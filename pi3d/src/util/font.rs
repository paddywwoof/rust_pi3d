extern crate ndarray as nd;
extern crate rusttype;

use rusttype::{point, Font, Scale};
use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use ::util::resources;

const TEX_SZ: usize = 1024;

pub struct GlyphTable {
    pub w: f32,
    pub h: f32,
    pub uv: [[f32; 2]; 4],
    pub verts: [[f32; 3]; 4],
    pub x: f32, //
    pub y: f32,
}

pub struct TextureFont {
    pub tex: ::texture::Texture,
    pub glyph_table: HashMap<char, GlyphTable>,
    pub height: f32,
    pub size: f32,
}

pub fn create(file_name: &str, glyphs: &str,
              add_glyphs: &str, size: f32) -> TextureFont {

    let grid_n = TEX_SZ / (size as usize); //TODO magic numbers!
    // Load the font
    let path_buf = resources::resource_name_to_path(file_name);
    let mut f = File::open(path_buf).expect("file not found");
    let mut contents = Vec::new();
    f.read_to_end(&mut contents)
        .expect("something went wrong reading the file");
    let font =
        Font::from_bytes(contents).expect("Error constructing Font");

    let mut image = nd::Array3::<u8>::zeros((TEX_SZ, TEX_SZ, 4));

    let scale = Scale { x: size, y: size };
    //TODO space needed
    let glyph_list = if glyphs == "" {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=~!@#$%^&*()_+[]\\{}|;':,./<>?\""
        } else {glyphs};
    let glyph_list = [glyph_list, add_glyphs].concat();

    let f_v_metrics = font.v_metrics(scale);
    let v_step = f_v_metrics.ascent - f_v_metrics.descent; // +ve is up here!
    //println!("{:?}", f_v_metrics);
    let mut cpoint = point(size * 0.5, size);
    //let mut advance = 0.0;
    let mut glyph_table = HashMap::<char, GlyphTable>::new();
    // put in a space as no pixel_bounding_box
    glyph_table.insert(' ', GlyphTable {w: size * 0.2, h: 1.0,
            uv: [[0.01, 0.0], [0.0, 0.0], [0.0, 0.01], [0.01, 0.01]],
            verts: [[0.01, 0.0, 0.0],[0.0, 0.0, 0.0],[0.0, -0.01, 0.0],[0.01, -0.01, 0.0]],
            x: 0.0, y: 0.01,});
    for (i, c) in glyph_list.chars().enumerate() {
        let g = font.glyph(c);
        let g = g.scaled(scale);
        if let Some(g_b_box) = g.exact_bounding_box() { // only do this if there's a glyph
            cpoint.x = ((i + 1) % grid_n) as f32 * size + (size - g_b_box.max.x - g_b_box.min.x) * 0.5;
            cpoint.y = ((i + 1) / grid_n + 1) as f32 * size; //TODO magic numbers
            let g = g.positioned(cpoint);
            if let Some(p_b_box) = g.pixel_bounding_box() { //TODO error catching
                g.draw(|x, y, v| {
                        let px = (x + p_b_box.min.x as u32) as usize;
                        let py = (y + p_b_box.min.y as u32) as usize;
                        for j in 0..3 {
                            image[[py, px, j]] = 255;
                        }
                        image[[py, px, 3]] = (v * 255.0) as u8;
                    });
                let cwidth = g_b_box.max.x - g_b_box.min.x;
                let cheight = g_b_box.max.y - g_b_box.min.y;
                let xscl = (cpoint.x + g_b_box.min.x) / TEX_SZ as f32;
                let yscl = (cpoint.y + g_b_box.min.y) / TEX_SZ as f32;
                let tw = cwidth / TEX_SZ as f32;
                let th = cheight / TEX_SZ as f32;
                let gt = GlyphTable {
                            w: cwidth,
                            h: cheight,
                            uv: [[xscl + tw, yscl + th], [xscl, yscl + th], [xscl, yscl], [xscl + tw, yscl]],
                            verts: [[cwidth, -g_b_box.max.y, 0.0],
                                    [0.0, -g_b_box.max.y, 0.0],
                                    [0.0,  cheight - g_b_box.max.y, 0.0],
                                    [cwidth, cheight - g_b_box.max.y, 0.0]],
                            x: xscl,
                            y: yscl,
                        };
                glyph_table.insert(c, gt);
            }
        }
    }
    let tex = ::texture::create_from_array(image);
    TextureFont {
        tex,
        glyph_table,
        height: v_step,
        size,
    }
}