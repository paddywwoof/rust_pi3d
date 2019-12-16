extern crate ndarray;

use std::collections::HashMap;
use std::str::FromStr;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use ndarray as nd;

struct Face {
    vertex: Vec<i32>,
    uv: Vec<i32>,
    normal: Vec<i32>,
}

pub fn parse_vertex(text: &str) -> (i32, i32, i32) {
    /*Parse text chunk specifying single vertex.
    * Possible formats:
    *  vertex index
    *  vertex index / texture index
    *  vertex index / texture index / normal index
    *  vertex index / / normal index
    */
    let chunks: Vec<&str> = text.split("/").collect();
    let v = i32::from_str_radix(chunks[0].trim(), 10).unwrap();
    let t = if chunks.len() > 1 {
        match i32::from_str_radix(chunks[1].trim(), 10) {
            Ok(x) => x,
            _ => 0i32,
        }
    } else {0};
    let n = if chunks.len() > 2 {
        match i32::from_str_radix(chunks[2].trim(), 10) {
            Ok(x) => x,
            _ => 0i32,
        }
    } else {0};
    (v, t, n)
}

pub fn create(cam: Rc<RefCell<::camera::CameraInternals>>,
              file_name: &str) -> (::shape::Shape, HashMap<String, ::texture::Texture>) {
    /*Loads an obj file with associated mtl file to produce Buffer object
    as part of a Shape. Arguments:
    */
    let mut bufs: Vec<::buffer::Buffer> = vec![];
    let mut vertices: Vec<f32> = vec![];
    let mut normals: Vec<f32> = vec![];
    let mut uvs: Vec<f32> = vec![];

    let mut faces = HashMap::<String, Vec<Face>>::new();

    let mut materials = Vec::<String>::new();
    let mut material: String = "".to_string();
    let mut mtllib: String = "".to_string();

    let res = ::util::resources::from_exe_path().unwrap();
    let path_buf = res.resource_name_to_path(file_name);
    //if !path_buf.is_file() {return Err(Error::MissingResource);} //TODO error catching
    let file = fs::File::open(path_buf).unwrap();
    let file = io::BufReader::new(file);
    for l in file.lines() {
        let l_string = l.unwrap();
        let chunks: Vec<String> = l_string.split_whitespace().map(|x| x.to_string()).collect();
        if chunks.len() > 0 {
            //# Vertices as (x,y,z) coordinates
            //# v 0.123 0.234 0.345
            if chunks[0] == "v" && chunks.len() >= 4 {
                vertices.push(f32::from_str(&chunks[1]).unwrap()); // x TODO no error catching (maybe not worth it here for speed)
                vertices.push(f32::from_str(&chunks[2]).unwrap()); // y
                vertices.push(-f32::from_str(&chunks[3]).unwrap()); // z away
            }
            //# Normals in (x, y, z) form; normals might not be unit
            //# vn 0.707 0.000 0.707
            if chunks[0] == "vn" && chunks.len() >= 4 {
                normals.push(f32::from_str(&chunks[1]).unwrap()); // x
                normals.push(f32::from_str(&chunks[2]).unwrap()); // y
                normals.push(-f32::from_str(&chunks[3]).unwrap()); // z -ve too
            }
            //# Texture coordinates in (u,v)
            //# vt 0.500 -1.352
            if chunks[0] == "vt" && chunks.len() >= 3 {
                uvs.push(f32::from_str(&chunks[1]).unwrap()); // u
                uvs.push(f32::from_str(&chunks[2]).unwrap()); // v
            }
            //# Face see comments in fn parse_vertex()
            if chunks[0] == "f" && chunks.len() >= 4 {
                let mut vertex_index: Vec<i32> = vec![];
                let mut uv_index: Vec<i32> = vec![];
                let mut normal_index: Vec<i32> = vec![];
            

                //# Precompute vert / normal / uv lists
                //# for negative index lookup
                let vertlen = vertices.len() as i32 / 3 + 1; // 3 entries in Vec per vert
                let normlen = normals.len() as i32 / 3+ 1; // 3 per norm
                let uvlen = uvs.len() as i32 / 2 + 1; // 2 per uv

                for i in 1..chunks.len() { // could be variable sides of polygon
                    let (v, t, n) = parse_vertex(&chunks[i]);
                    if v != 0 { // should always be true otherwise numv will be out
                        vertex_index.push(
                                if v < 0 {v + vertlen} else { v }
                            );
                    }
                    if t != 0 {
                        uv_index.push(
                                if t < 0 {t + uvlen} else { t }
                            );
                    }
                    if n != 0 {
                        normal_index.push(
                                if n < 0 {n + normlen} else { n }
                            );
                    }
                }
                //{ // block for f_mc to exist
                    //let f_mc = faces.entry(mcurrent).or_insert(vec![]);
                    let f_mc = faces.entry(material.to_string()).or_insert(vec![]);
                    f_mc.push(Face{vertex: vertex_index,
                                   uv: uv_index,
                                   normal: normal_index,
                                   });
                //}
            }

            //# Materials definition
            if chunks[0] == "mtllib" && chunks.len() == 2 {
                mtllib = chunks[1].to_string(); // can't borrow ref
            }

            //# Material
            if chunks[0] == "usemtl" {
                if chunks.len() > 1 {
                    material = chunks[1].to_string(); // can't borrow ref
                } else {
                    material = "".to_string();
                }
            }
        }
    }    
    // should now have material file name in mtllib

    // create the buffers, one for each material g
    for (m, face) in faces.iter() { // m is material, face is Vec of Face structs
        let mut m_vertices: Vec<f32> = vec![]; //TODO better to go into ndarray at this stage?
        let mut m_normals: Vec<f32> = vec![];
        let mut m_tex_coords: Vec<f32> = vec![];
        let mut m_faces: Vec<u16> = vec![];

        let mut i: usize = 0; //# vertex counter in this material
        // vert_map checks for reuse of vertex/uv/normal in
        let mut vert_map: HashMap<(i32, i32, i32), usize> = HashMap::new();

        for f in face.iter() {
            let length = f.vertex.len();
            let length_n = f.normal.len();
            let length_uv = f.uv.len();
            let mut vert_vec: Vec<usize> = vec![]; // will hold index to m_vertices

            for v in 0..length {
                //vert_tuple (v,n,u) with -1 if n or u missing check in vert_map and
                let vert_tuple = (f.vertex[v],
                        if length_n == length {f.normal[v]} else {-1},
                        if length_uv > 0 {f.uv[v]} else {-1});
                if vert_map.contains_key(&vert_tuple) {
                    vert_vec.push(*vert_map.get(&vert_tuple).unwrap());
                } else {
                    //only add here if doesn't already exist
                    for vi in 0..3 { // xyz components
                        m_vertices.push(vertices[(f.vertex[v] as usize - 1) * 3 + vi]);
                        if length_n == length { //#only use normals if there is one for each vertex
                            m_normals.push(normals[(f.normal[v] as usize - 1) * 3 + vi]);
                        }
                    }
                    if length_uv > 0  { //&& uvs[f.uv[v] - 1].len() == 2) {
                        for vi in 0..2 {
                            m_tex_coords.push(uvs[(f.uv[v] as usize - 1) * 2 + vi]);
                        }
                    }
                    vert_map.insert(vert_tuple, i);
                    vert_vec.push(i);
                    i += 1;
                }
            }
            for t in 0..(vert_vec.len() - 2) {
                m_faces.push(vert_vec[0] as u16);
                m_faces.push(vert_vec[t + 2] as u16);
                m_faces.push(vert_vec[t + 1] as u16);
            }
        }
        // finally add a sacrificial line TODO, this could be done in buffer
        for _vi in 0..3 {
            m_vertices.push(0.0);
            m_normals.push(0.0);
        }
        for _vi in 0..2 {
            m_tex_coords.push(0.0);
        }

        let calc_normals = if m_normals.len() == m_vertices.len() {
                false} else {true
        };

        bufs.push(::buffer::create(&::shader::Program::new(),
                nd::Array::from_shape_vec((m_vertices.len() / 3, 3usize), m_vertices).unwrap(), //TODO make functions return Result and feedback errors
                nd::Array::from_shape_vec((m_normals.len() / 3, 3usize), m_normals).unwrap(),
                nd::Array::from_shape_vec((m_tex_coords.len() / 2, 2usize), m_tex_coords).unwrap(),
                nd::Array::from_shape_vec((m_faces.len() / 3, 3usize), m_faces).unwrap(), calc_normals));
        materials.push(m.to_string());
    }
    println!("materials len:  {:?}, mtllib: {:?}", materials.len(), mtllib);

    // parse mtllib - pi3d only uses Kd and map_Kd
    let mut mtl_ref: String = "".to_string(); // set before each set of specifications
    let mut color_diffuse = HashMap::<String, [f32; 3]>::new(); // map material name to RGB
    let mut map_diffuse = HashMap::<String, String>::new(); // map material name to image file name
    let mut tex_list = HashMap::<String, ::texture::Texture>::new(); // map image file name to Texture

    let mut file_path = PathBuf::from(&file_name);
    file_path.pop(); // now the parent, without filename.

    let mut tmp_f = file_path.clone();
    tmp_f.push(mtllib); //
    let path_buf = res.resource_name_to_path(tmp_f.to_str().unwrap());
    let file = fs::File::open(path_buf).unwrap(); //TODO error checking
    let file = io::BufReader::new(file);
    for l in file.lines() {
        let l_string = l.unwrap();
        let chunks: Vec<String> = l_string.split_whitespace().map(|x| x.to_string()).collect();
        if chunks.len() > 0 {
            if chunks[0] == "newmtl" && chunks.len() == 2 {
                mtl_ref = chunks[1].to_string();
            }
            if chunks[0] == "Kd" && chunks.len() >= 4 {
                color_diffuse.insert(mtl_ref.to_string(),
                    [f32::from_str(&chunks[1]).unwrap(),
                     f32::from_str(&chunks[2]).unwrap(),
                     f32::from_str(&chunks[3]).unwrap()]);
            }
            if chunks[0] == "map_Kd" && chunks.len() >= 2 {
                map_diffuse.insert(mtl_ref.to_string(), chunks[1].to_string());
                if !tex_list.contains_key(&chunks[1]) {
                    let mut tmp_f = file_path.clone();
                    tmp_f.push(&chunks[1]);
                    let mut tex = ::texture::create_from_file(tmp_f.to_str().unwrap());
                    tex.flip_image(true, false);
                    tex_list.insert(chunks[1].to_string(), tex);
                }
            }
        }
    }

    let mut model = ::shape::create(bufs, cam);
    for i in 0..model.buf.len() {
        // buf number -> material name -> image file -> Texture struct
        if i < materials.len() {
            let mat_key = &materials[i];
            if map_diffuse.contains_key(mat_key) && tex_list.contains_key(&map_diffuse[mat_key]) {
                model.buf[i].set_textures(&vec![tex_list[&map_diffuse[mat_key]].id]);
            }
            // buf number  -> material name -> RGB array
            if color_diffuse.contains_key(mat_key) {
                model.buf[i].set_material(&color_diffuse[mat_key]);
            }
        }
    }
    (model, tex_list)
}
