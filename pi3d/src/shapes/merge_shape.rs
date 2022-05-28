extern crate ndarray;
extern crate rand;

use ndarray as nd;
use std::cell::RefCell;
use std::f32::consts;
use std::rc::Rc;

use util::vec3::{rotate_vec, rotate_vec_slice};

/// add a Vec of Buffers at specific location, rotation, scales to an existing
/// Shape
///
/// * `merge_to` existing shape to merge new shapes to
/// * `new_bufs` Vec of &Buffer objects to build into merge_to
/// * `loc` Vec of [x, y, z] locations matching each Buffer in new_bufs
/// * `rot` Vec of [rx, ry, rz] rotations
/// * `scl` Vec of [sx, sy, sz] scales
/// * `num` Vec of index values to merge_to.buf[] for merging new Buffers
///
pub fn add_buffers(
    merge_to: &mut ::shape::Shape,
    new_bufs: Vec<&::buffer::Buffer>,
    loc: Vec<[f32; 3]>,
    rot: Vec<[f32; 3]>,
    scl: Vec<[f32; 3]>,
    num: Vec<usize>,
) {
    if new_bufs.len() != loc.len() || loc.len() != rot.len() || rot.len() != scl.len() {
        panic!("\n\nadd_buffers() needs four Vecs of same size\n\n");
    }
    // make note of num of verts and faces already there.
    // also have a new mut num_v, verts, norms, tex_coord and faces for each num
    let mut num_v = Vec::<u16>::new();
    let mut verts = Vec::<nd::Array2<f32>>::new();
    let mut norms = Vec::<nd::Array2<f32>>::new();
    let mut tex_coords = Vec::<nd::Array2<f32>>::new();
    let mut faces = Vec::<nd::Array2<u16>>::new();
    let mut new_buf_ix = Vec::<usize>::new();
    for i in 0..merge_to.buf.len() {
        num_v.push(merge_to.buf[i].array_buffer.shape()[0] as u16);
        verts.push(merge_to.buf[i].array_buffer.slice(s![.., 0..3]).to_owned());
        norms.push(merge_to.buf[i].array_buffer.slice(s![.., 3..6]).to_owned());
        tex_coords.push(merge_to.buf[i].array_buffer.slice(s![.., 6..8]).to_owned());
        faces.push(merge_to.buf[i].element_array_buffer.to_owned());
        new_buf_ix.push(0);
    }
    for i in 0..new_bufs.len() {
        // if num >= merge_to.buf.len() -> add new empty buffers to bring to size
        while num[i] >= merge_to.buf.len() {
            merge_to.buf.push(::buffer::create_empty());
            num_v.push(0); // and crete empty buffers
            verts.push(
                merge_to.buf[0]
                    .array_buffer
                    .slice(s![0..0, 0..3])
                    .to_owned(),
            );
            norms.push(
                merge_to.buf[0]
                    .array_buffer
                    .slice(s![0..0, 3..6])
                    .to_owned(),
            );
            tex_coords.push(
                merge_to.buf[0]
                    .array_buffer
                    .slice(s![0..0, 6..8])
                    .to_owned(),
            );
            faces.push(
                merge_to.buf[0]
                    .element_array_buffer
                    .slice(s![0..0, ..])
                    .to_owned(),
            );
            new_buf_ix.push(0);
        }
        // scale then rotate new verts then add displacement
        let new_verts = &new_bufs[i].array_buffer.slice(s![.., 0..3]) * &nd::arr1(&scl[i]);
        let new_verts = rotate_vec(&rot[i], &new_verts) + &nd::arr1(&loc[i]);
        // then add them to existing verts
        verts[num[i]] = nd::stack(nd::Axis(0), &[verts[num[i]].view(), new_verts.view()]).unwrap();
        // rotate new normals
        let new_norms = rotate_vec_slice(&rot[i], &new_bufs[i].array_buffer.slice(s![.., 3..6]));
        // then add them to existing normals
        norms[num[i]] = nd::stack(nd::Axis(0), &[norms[num[i]].view(), new_norms.view()]).unwrap();
        // stack tex_coords
        tex_coords[num[i]] = nd::stack(
            nd::Axis(0),
            &[
                tex_coords[num[i]].view(),
                new_bufs[i].array_buffer.slice(s![.., 6..8]),
            ],
        )
        .unwrap();
        // add num_v to values in faces
        faces[num[i]] = nd::stack(
            nd::Axis(0),
            &[
                faces[num[i]].view(),
                (&new_bufs[i].element_array_buffer + num_v[num[i]]).view(),
            ],
        )
        .unwrap();
        num_v[num[i]] += new_verts.shape()[0] as u16;
        new_buf_ix[num[i]] = i;
    }
    for i in 0..merge_to.buf.len() {
        let mut extended_buf = ::buffer::create(
            &::shader::Program::new(),
            verts[i].to_owned(),
            norms[i].to_owned(),
            tex_coords[i].to_owned(),
            faces[i].to_owned(),
            false,
        );
        // copy over shader, textures, unib, draw_method
        let ix = new_buf_ix[i];
        extended_buf.unib = new_bufs[ix].unib.clone();
        extended_buf.draw_method = new_bufs[ix].draw_method;
        extended_buf.shader_id = new_bufs[ix].shader_id;
        extended_buf.attribute_names = new_bufs[ix].attribute_names.clone();
        extended_buf.attribute_values = new_bufs[ix].attribute_values.clone();
        extended_buf.uniform_names = new_bufs[ix].uniform_names.clone();
        extended_buf.uniform_values = new_bufs[ix].uniform_values.clone();
        extended_buf.textures = new_bufs[ix].textures.clone();
        merge_to.buf[i] = extended_buf;
    }
}

/// wrapper round add_buffers that can be passed a Vec of Shape objects
/// and will the buf Vec and add all the Buffer objects
///
pub fn add_shapes(
    merge_to: &mut ::shape::Shape,
    new_shapes: Vec<&::shape::Shape>,
    loc: Vec<[f32; 3]>,
    rot: Vec<[f32; 3]>,
    scl: Vec<[f32; 3]>,
    num: Vec<usize>,
) {
    let mut bufs = Vec::<&::buffer::Buffer>::new();
    let mut new_loc = Vec::<[f32; 3]>::new();
    let mut new_rot = Vec::<[f32; 3]>::new();
    let mut new_scl = Vec::<[f32; 3]>::new();
    let mut new_num = Vec::<usize>::new();
    for i in 0..new_shapes.len() {
        for j in 0..new_shapes[i].buf.len() {
            bufs.push(&new_shapes[i].buf[j]);
            new_loc.push([loc[i][0], loc[i][1], loc[i][2]]);
            new_rot.push([rot[i][0], rot[i][1], rot[i][2]]);
            new_scl.push([scl[i][0], scl[i][1], scl[i][2]]);
            new_num.push(num[i]);
        }
    }
    add_buffers(merge_to, bufs, new_loc, new_rot, new_scl, new_num);
}

/// create a cluster of shapes on an elevation map
///
/// * `merge_to` existing Shape to merge cluster of duplicate shapes to
/// * `new_shape' new Shape to duplicate
/// * `map' elevation_map for calculating height
/// * `xpos`, `ypos` centre of cluster
/// * `w`, `d` width and depth of cluster
/// * `minscl`, `maxscl` range of scaling factors to apply
/// * `count` number of duplicates to make
///
pub fn cluster(
    merge_to: &mut ::shape::Shape,
    new_shape: &::shape::Shape,
    map: &::shapes::elevation_map::ElevationMap,
    xpos: f32,
    zpos: f32,
    w: f32,
    d: f32,
    minscl: f32,
    maxscl: f32,
    count: usize,
) {
    let mut bufs = Vec::<&::buffer::Buffer>::new();
    let mut new_loc = Vec::<[f32; 3]>::new();
    let mut new_rot = Vec::<[f32; 3]>::new();
    let mut new_scl = Vec::<[f32; 3]>::new();
    let mut new_num = Vec::<usize>::new();
    for _i in 0..count {
        let x = xpos - 0.5 * w + w * rand::random::<f32>();
        let z = zpos - 0.5 * d + d * rand::random::<f32>();
        let (y, _norm) = map.calc_height(x, z);
        let ry = 2.0 * consts::PI * rand::random::<f32>();
        let scl = minscl + (maxscl - minscl) * rand::random::<f32>();
        for j in 0..new_shape.buf.len() {
            bufs.push(&new_shape.buf[j]);
            new_loc.push([x, y, z]);
            new_rot.push([0.0, ry, 0.0]);
            new_scl.push([scl, scl, scl]);
            new_num.push(j);
        }
    }

    add_buffers(merge_to, bufs, new_loc, new_rot, new_scl, new_num);
}

/// initial creation produces a shape with an empty buffer
///
pub fn create(cam: Rc<RefCell<::camera::CameraInternals>>) -> ::shape::Shape {
    let new_buffer = ::buffer::create_empty();
    ::shape::create(vec![new_buffer], cam)
}

//TODO pub fn radial_copy();
