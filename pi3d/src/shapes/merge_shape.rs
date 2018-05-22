extern crate ndarray;

use ndarray as nd;

use ::util::vec3::{rotate_vec, rotate_vec_slice};

pub fn add_buffers(shape: &mut ::shape::Shape, new_bufs: Vec<&::buffer::Buffer>,
                    loc: Vec<&[f32; 3]>, rot: Vec<&[f32; 3]>, scl: Vec<&[f32; 3]>,
                    num: Vec<usize>) {
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
    for i in 0..shape.buf.len() {
        num_v.push(shape.buf[i].array_buffer.shape()[0] as u16);
        verts.push(shape.buf[i].array_buffer.slice(s![..,0..3]).to_owned());
        norms.push(shape.buf[i].array_buffer.slice(s![..,3..6]).to_owned());
        tex_coords.push(shape.buf[i].array_buffer.slice(s![..,6..8]).to_owned());
        faces.push(shape.buf[i].element_array_buffer.to_owned());
        new_buf_ix.push(0);
    }
    for i in 0..new_bufs.len() {
        // if num >= shape.buf.len() -> add new empty buffers to bring to size
        while num[i] >= shape.buf.len() {
            shape.buf.push(::buffer::create_empty());
            num_v.push(0); // and crete empty buffers
            verts.push(shape.buf[0].array_buffer.slice(s![0..0,0..3]).to_owned());
            norms.push(shape.buf[0].array_buffer.slice(s![0..0,3..6]).to_owned());
            tex_coords.push(shape.buf[0].array_buffer.slice(s![0..0,6..8]).to_owned());
            faces.push(shape.buf[0].element_array_buffer.slice(s![0..0,..]).to_owned());
            new_buf_ix.push(0);
        }
        // scale then rotate new verts then add displacement
        let new_verts = &new_bufs[i].array_buffer.slice(s![.., 0..3]) * &nd::arr1(scl[i]);
        let new_verts = rotate_vec(rot[i], &new_verts) + &nd::arr1(loc[i]);
        // then add them to existing verts
        verts[num[i]] = nd::stack(nd::Axis(0),
                        &[verts[num[i]].view(),
                          new_verts.view()]).unwrap();
        // rotate new normals
        let new_norms = rotate_vec_slice(rot[i], &new_bufs[i].array_buffer.slice(s![.., 3..6]));
        // then add them to existing normals
        norms[num[i]] = nd::stack(nd::Axis(0),
                        &[norms[num[i]].view(),
                          new_norms.view()]).unwrap();
        // stack tex_coords
        tex_coords[num[i]] = nd::stack(nd::Axis(0),
                        &[tex_coords[num[i]].view(),
                          new_bufs[i].array_buffer.slice(s![.., 6..8])
                         ]).unwrap();
        // add num_v to values in faces
        faces[num[i]] = nd::stack(nd::Axis(0),
                        &[faces[num[i]].view(),
                          (&new_bufs[i].element_array_buffer + num_v[num[i]]).view()
                         ]).unwrap();
        num_v[num[i]] += new_verts.shape()[0] as u16;
        new_buf_ix[num[i]] = i;
    }
    for i in 0..shape.buf.len() {
        let mut extended_buf = ::buffer::create(&::shader::Program::new(),
                    verts[i].to_owned(), norms[i].to_owned(), tex_coords[i].to_owned(), faces[i].to_owned(), false);
        // copy over shader, textures, unib, draw_method
        let ix = new_buf_ix[i];
        extended_buf.unib = new_bufs[ix].unib.clone();
        extended_buf.draw_method = new_bufs[ix].draw_method.clone();
        extended_buf.shader_id = new_bufs[ix].shader_id.clone();
        extended_buf.attribute_names = new_bufs[ix].attribute_names.clone();
        extended_buf.attribute_values = new_bufs[ix].attribute_values.clone();
        extended_buf.uniform_names = new_bufs[ix].uniform_names.clone();
        extended_buf.uniform_values = new_bufs[ix].uniform_values.clone();
        extended_buf.textures = new_bufs[ix].textures.clone();
        shape.buf[i] = extended_buf;
    }
}

pub fn add_shapes(shape: &mut ::shape::Shape, new_shapes: Vec<&::shape::Shape>,
                    loc: Vec<&[f32; 3]>, rot: Vec<&[f32; 3]>, scl: Vec<&[f32; 3]>,
                    num: Vec<usize>) {
    let mut bufs = Vec::<&::buffer::Buffer>::new();
    let mut new_loc = Vec::<&[f32; 3]>::new();
    let mut new_rot = Vec::<&[f32; 3]>::new();
    let mut new_scl = Vec::<&[f32; 3]>::new();
    let mut new_num = Vec::<usize>::new();
    for i in 0..new_shapes.len() {
        for j in 0..new_shapes[i].buf.len() {
            bufs.push(&new_shapes[i].buf[j]);
            new_loc.push(&loc[i]);
            new_rot.push(&rot[i]);
            new_scl.push(&scl[i]);
            new_num.push(num[i]);
        }
    }
    add_buffers(shape, bufs, new_loc, new_rot, new_scl, new_num);
}

pub fn create() -> ::shape::Shape {
    let new_buffer = ::buffer::create_empty();
    ::shape::create(vec![new_buffer])
}

//TODO pub fn cluster(); pub fn radial_copy();
