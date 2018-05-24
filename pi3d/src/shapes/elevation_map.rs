extern crate ndarray;
extern crate image;

use ndarray as nd;

pub fn create(disp: &::display::Display, mapfile: &str, width: f32, depth: f32,
              height: f32, ix: usize, iy: usize, ntiles: f32, _texmap: &str) -> ::shape::Shape {

    let ix = if ix < 200 {ix + 1} else {200}; // one more vertex in each direction than number of divisions
    let iy = if iy < 200 {iy + 1} else {200};
    let f = disp.res.resource_name_to_path(mapfile);
    //println!("f={:?}", f);
    let im = image::open(f).unwrap();
    // convert to Gray
    let mut im = image::imageops::colorops::grayscale(&im);
    // resize
    let (w, h) = im.dimensions();
    if w != ix as u32 || h != iy as u32 {
        im = image::imageops::resize(&im, ix as u32, iy as u32, image::FilterType::Lanczos3);
    }
    // flip top to bottom and left to right - which results in 180 degree rotation
    let im = image::imageops::rotate180(&im);
    let pixels = nd::Array::from_shape_vec((iy, ix, 1), im.into_raw()).unwrap();

    //TODO texmap used for mapping other info into uv coords (integer part)

    let wh = width * 0.5;
    let hh = depth * 0.5;
    let ws = width / (ix as f32 - 1.0);
    let hs = depth / (iy as f32 - 1.0);
    let ht = height / 255.0;
    let tx = 1.0 * ntiles / ix as f32;
    let ty = 1.0 * ntiles / iy as f32;

    let mut verts = Vec::<f32>::new();
    let mut faces = Vec::<u16>::new();
    let mut tex_coords = Vec::<f32>::new();

    for y in 0..iy {
        for x in 0..ix {
            //println!("y,x {:?},{:?}", y, x);
            verts.extend_from_slice(&[-wh + x as f32 * ws,
                                       pixels[[y, x, 0]] as f32 * ht, 
                                      -hh + y as f32 * hs]);
            tex_coords.extend_from_slice(&[(ix - x) as f32 * tx, (iy - y) as f32 * ty]);
        }
    }

    //create one long triangle_strip by alternating X directions
    for y in 0..(iy - 1) {
        for x in 0..(ix - 1) {
            let i = (y * ix) + x;
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
    ::shape::create(vec![new_buffer])
}

pub fn calc_height(map: &::shape::Shape, px: f32, pz: f32) -> (f32, Vec<f32>) {
    //TODO a) for regular map this doesn't need to iterate through whole thing
    // b) Buffer.calc_normals does cross product calc already so should save result
    let px = px - map.unif[[0, 0]];
    let pz = pz - map.unif[[0, 2]];
    for f in map.buf[0].element_array_buffer.axis_iter(nd::Axis(0)) {
        let x0 = map.buf[0].array_buffer[[f[[0]] as usize, 0]];
        let z0 = map.buf[0].array_buffer[[f[[0]] as usize, 2]];
        let x1 = map.buf[0].array_buffer[[f[[1]] as usize, 0]];
        let z1 = map.buf[0].array_buffer[[f[[1]] as usize, 2]];
        let x2 = map.buf[0].array_buffer[[f[[2]] as usize, 0]];
        let z2 = map.buf[0].array_buffer[[f[[2]] as usize, 2]];
        if ((z1 - z0) * (px - x0) + (-x1 + x0) * (pz - z0) >= 0.0) &&
           ((z2 - z1) * (px - x1) + (-x2 + x1) * (pz - z1) >= 0.0) &&
           ((z0 - z2) * (px - x2) + (-x0 + x2) * (pz - z2) >= 0.0) {
            let v0 = nd::arr1(&[x0, map.buf[0].array_buffer[[f[[0]] as usize, 1]], z0]);
            let v1 = nd::arr1(&[x1, map.buf[0].array_buffer[[f[[1]] as usize, 1]], z1]);
            let v2 = nd::arr1(&[x2, map.buf[0].array_buffer[[f[[2]] as usize, 1]], z2]);
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
