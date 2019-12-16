extern crate ndarray;
extern crate image;

use ndarray as nd;

// create a new struct definition 
make_shape!(ElevationMap, ix:f32=0.0, iz:f32=0.0, width:f32=200.0, depth:f32=200.0);

/// generate an elevation map Shape
///
/// * `disp` reference to the display object which has file path functionality
/// * `mapfile` relative path to image file representing heights
/// * `width` edge to edge in x direction
/// * `depth` in z direction
/// * `height` in y direction scaled by pixel values 0 to 255
/// * `ix` number of polygons in x direction 
/// * `iz` polygons in z direction
/// * `ntiles` uv repeats for texture mapping
/// * `_texmap` TODO used for allocating multiple textures to the map
/// NB for no image scaling the mapfile should be one more pixel than ix and iz
/// i.e. 33x33 image for 32x32 squares in grid
/// 
/// TODO put this as class method (i.e. ::new())
pub fn new_map(cam: Rc<RefCell<::camera::CameraInternals>>,
               mapfile: &str, width: f32, depth: f32, height: f32, ix: usize, iz: usize,
               ntiles: f32, _texmap: &str) -> ElevationMap {

    let ix = if ix < 200 {ix + 1} else {200}; // one more vertex in each direction than number of divisions
    let iz = if iz < 200 {iz + 1} else {200};
    let res = ::util::resources::from_exe_path().unwrap();
    let f = res.resource_name_to_path(mapfile);
    //println!("f={:?}", f);
    let im = image::open(f).unwrap();
    // convert to Gray
    let mut im = image::imageops::colorops::grayscale(&im);
    // resize
    let (w, h) = im.dimensions();
    if w != ix as u32 || h != iz as u32 {
        im = image::imageops::resize(&im, ix as u32, iz as u32, image::FilterType::Lanczos3);
    }
    // flip top to bottom and left to right - which results in 180 degree rotation
    let im = image::imageops::rotate180(&im);
    let pixels = nd::Array::from_shape_vec((iz, ix, 1), im.into_raw()).unwrap();

    //TODO texmap used for mapping other info into uv coords (integer part)

    let wh = width * 0.5;
    let hh = depth * 0.5;
    let ws = width / (ix as f32 - 1.0);
    let hs = depth / (iz as f32 - 1.0);
    let ht = height / 255.0;
    let tx = 1.0 * ntiles / ix as f32;
    let tz = 1.0 * ntiles / iz as f32;

    let mut verts = Vec::<f32>::new();
    let mut faces = Vec::<u16>::new();
    let mut tex_coords = Vec::<f32>::new();

    for z in 0..iz {
        for x in 0..ix {
            //println!("z,x {:?},{:?}", z, x);
            verts.extend_from_slice(&[-wh + x as f32 * ws,
                                       pixels[[z, x, 0]] as f32 * ht, 
                                      -hh + z as f32 * hs]);
            tex_coords.extend_from_slice(&[(ix - x) as f32 * tx, (iz - z) as f32 * tz]);
        }
    }

    //create one long triangle_strip by alternating X directions
    for z in 0..(iz - 1) {
        for x in 0..(ix - 1) {
            let i = (z * ix) + x;
            faces.extend_from_slice(&[i as u16, (i + ix) as u16, (i + ix + 1) as u16]);
            faces.extend_from_slice(&[(i + ix + 1) as u16, (i + 1) as u16, i as u16]);
        }
    }

    let nverts = verts.len() / 3;
    let nfaces = faces.len() / 3;
    let new_buffer = ::buffer::create(&::shader::Program::new(),
                nd::Array::from_shape_vec((nverts, 3usize), verts).unwrap(),
                nd::Array2::<f32>::zeros((0, 3)),
                nd::Array::from_shape_vec((nverts, 2usize), tex_coords).unwrap(),
                nd::Array::from_shape_vec((nfaces, 3usize), faces).unwrap(), true);
    let mut new_shape = ::shapes::elevation_map::create(vec![new_buffer], cam);
    new_shape.ix = ix as f32 - 1.0; // hold these for calc_height in additional attributes
    new_shape.iz = iz as f32 - 1.0;
    new_shape.width = width;
    new_shape.depth = depth;
    new_shape
}

/// find the value of the y component of the location on the triangle
/// where the x, z location lies within the elevation_map
///
/// * `map` the Shape object representing the elevation map
/// * `px` x location in world space
/// * `pz` z location
///
/// returns a tuple of the (height, normal vector at that point) this can
/// be used for resistance to movement, bouncing etc.
/// 
/// TODO put this as a class method
pub fn calc_height(map: &ElevationMap, px: f32, pz: f32) -> (f32, Vec<f32>) {
    //TODO a) the skip method will only work for regular maps
    // b) Buffer.calc_normals does cross product calc already so should save result
    // in Buffer on creation
    let px = px - map.unif[[0, 0]];
    let pz = pz - map.unif[[0, 2]];
    let skip_n = (((pz + map.depth * 0.5) * map.iz / map.depth).floor() * map.ix * 2.0
                  + ((px + map.width * 0.5) * map.ix / map.width).floor() * 2.0) as usize;
    for f in map.buf[0].element_array_buffer.axis_iter(nd::Axis(0)).skip(skip_n) {
        let mut v: Vec<Vec<f32>> = vec![vec![0.0; 3]; 3];
        for i in 0..3 { // the three vertices of this element
            for j in 0..3 { // the x, y, z components of each vertex
                v[i][j] = map.buf[0].array_buffer[[f[[i]] as usize, j]];
            }
        }
        if ((v[1][2] - v[0][2]) * (px - v[0][0]) + (-v[1][0] + v[0][0]) * (pz - v[0][2]) >= 0.0) &&
           ((v[2][2] - v[1][2]) * (px - v[1][0]) + (-v[2][0] + v[1][0]) * (pz - v[1][2]) >= 0.0) &&
           ((v[0][2] - v[2][2]) * (px - v[2][0]) + (-v[0][0] + v[2][0]) * (pz - v[2][2]) >= 0.0) {
            let v0 = nd::arr1(&v[0]);
            let v1 = nd::arr1(&v[1]);
            let v2 = nd::arr1(&v[2]);
            //calc normal from two edge vectors v2-v1 and v3-v1
            let n_vec = ::util::vec3::cross(&(&v1 - &v0), &(v2 - v0));
            //equation of plane: Ax + By + Cz = k_val where A,B,C are components of normal. x,y,z for point v1 to find k_val
            let k_val = ::util::vec3::dot(&n_vec, &v1);
            //return y val i.e. y = (k_val - Ax - Cz)/B also the normal vector seeing as this has been calculated
            return ((k_val - n_vec[[0]] * px - n_vec[[2]] * pz) / n_vec[[1]], n_vec.to_vec());
        }
    }
    //TODO fn should return Option<> and need to be unwrapped rather than return something
    (0.0, vec![0.0, 1.0, 0.0])
}
