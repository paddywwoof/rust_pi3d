extern crate ndarray;

use std::rc::Rc;
use std::cell::RefCell;
use ndarray as nd;

const GAP: f32 = 1.0; // line spacing
const SPACE: f32 = 0.03; // between char (proportion of line space)
const NORMALS: [[f32; 3]; 4] = [[0.0, 0.0, -1.0]; 4];

pub fn create(cam: Rc<RefCell<::camera::CameraInternals>>,
              font: &::util::font::TextureFont, string: &str, justify: f32) -> ::shape::Shape {
    //TODO sort out reason for extra vertex (uv point)
    let mut verts = Vec::<f32>::new();
    let mut norms = Vec::<f32>::new();
    let mut tex_coords = Vec::<f32>::new();
    let mut faces = Vec::<u16>::new();

    let mut xoff = 0.0;
    let mut yoff = 0.0;
    let nlines = string.matches("\n").count() + 1;
    let default = &font.glyph_table[&' '];
    let mut temp_verts = Vec::<[f32;3]>::new();
    let mut lines = 0;
    let sx = 0.24 * 4.0; //TODO pass size argument?
    let sy = 0.24 * 4.0;
    let mut maxx = -10000.0;
    let mut minx = 10000.0;
    let mut maxy = -10000.0;
    let mut miny = 10000.0;

    let last_char = string.chars().count() - 1;
    for (i, c) in string.chars().enumerate() {
        if c != '\n' {
            let glyph = match &font.glyph_table.get(&c) {&Some(g) => g, _ => default};
            //let (w, h, texc, verts) = glyph[0:4]
            for j in glyph.verts.iter() {
                temp_verts.push([j[0] + xoff, j[1], j[2]]);
            }
            xoff += glyph.w + SPACE * font.height;
            for j in glyph.uv.iter() {
                tex_coords.push(j[0]);
                tex_coords.push(j[1]);
            }
            for j in NORMALS.iter() {
                for k in j.iter() {
                    norms.push(*k);
                }
            }
            //# Take Into account unprinted \n characters
            let stv = 4 * (i - lines) as u16;
            faces.push(stv); faces.push(stv + 1); faces.push(stv + 2);
            faces.push(stv + 2); faces.push(stv + 3); faces.push(stv);
        }
        if i == last_char || c == '\n' {
            let cx = xoff * justify;
            for j in 0..temp_verts.len() {
                let x = (temp_verts[j][0] - cx) * sx;
                let y = (temp_verts[j][1] + nlines as f32 * font.height * GAP / 2.0 - yoff) * sy;
                let z = temp_verts[j][2];
                if x < minx {minx = x;}
                if x > maxx {maxx = x;}
                if y < miny {miny = y;}
                if y > maxy {maxy = y;}
                verts.push(x); verts.push(y); verts.push(z);
            }
            yoff += font.height * GAP;
            xoff = 0.0;
            temp_verts.clear();
            lines += 1;
            continue; //don't attempt to draw this character!
        }
    }
    verts.append(&mut vec![0.0; 3]); //TODO why needs extra tex coord?
    norms.append(&mut vec![0.0; 3]);
    tex_coords.append(&mut vec![0.0; 2]);

    let nverts = verts.len() / 3;
    let nfaces = faces.len() / 3;
    let av_x = (maxx + minx) * 0.5;
    let av_y = (maxy + miny) * 0.5;
    for i in 0..nverts {
        verts[i * 3] -= av_x; // shift x values
        verts[i * 3 + 1] -= av_y; // shift y values and flip
    }
    let mut new_buffer = ::buffer::create(&::shader::Program::new(),
                nd::Array::from_shape_vec((nverts, 3usize), verts).unwrap(), //TODO make functions return Result and feedback errors
                nd::Array::from_shape_vec((nverts, 3usize), norms).unwrap(),
                nd::Array::from_shape_vec((nverts, 2usize), tex_coords).unwrap(),
                nd::Array::from_shape_vec((nfaces, 3usize), faces).unwrap(), false);
    new_buffer.set_textures(&vec![font.tex.id]);
    new_buffer.set_blend(true);
    ::shape::create(vec![new_buffer], cam)
}
