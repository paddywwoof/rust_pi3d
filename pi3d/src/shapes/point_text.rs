pub struct TextBlock {
    x: f32,
    y: f32,
    z: f32,
    capacity: usize,
    size: f32,
    text: String,
    rot: f32,
    char_rot: f32,
    spacing: char,
    space: f32,
    rgba: [f32; 4],
    justification: f32,
    start: usize,
    changed: bool,
}

pub struct PointText {
    pub points: ::shape::Shape,
    blocks: Vec<TextBlock>,
    max_chars: usize,
    point_size: f32,
}

impl PointText {
    //pub fn add_text_block(&mut self,
    pub fn add_text_block(&mut self, font: &::util::font::TextureFont,
                        position: &[f32; 3], capacity: usize, text: &str) -> usize {
        if text.chars().count() >= capacity {
            panic!("text won't fit into capacity for this TextBlock");
        }
        let start = match self.blocks.last() {
            Some(blk) => blk.start + blk.capacity,
            _ => 0, // it must have been an empty Vec
        };
        if (start + capacity) >= self.max_chars {
            panic!("TextBlock is going to overflow PointText");
        }
        let new_block = TextBlock {
            x: position[0],
            y: position[1],
            z: position[2],
            capacity,
            size: 0.99,
            text: text.to_string(),
            rot: 0.0,
            char_rot: 0.0,
            spacing: 'F',
            space: 0.05,
            rgba: [0.999; 4],
            justification: 0.0,
            start,
            changed: false,
        };
        self.blocks.push(new_block);
        let block_id = self.blocks.len() - 1;
        self.regen(font, block_id); 
        block_id
    }

    pub fn draw(&mut self, mut camera: &mut ::camera::Camera) {
        self.points.draw(&mut camera);
    }

    pub fn set_shader(&mut self, shader: &::shader::Program) {
        self.points.set_shader(shader);
    }

    // these all apply to one of the TextBlocks
    //pub fn set_position(&mut self,
    pub fn set_position(&mut self, font: &::util::font::TextureFont,
                                    block_id: usize, position: &[f32; 3]) {
        self.blocks[block_id].x = position[0];
        self.blocks[block_id].y = position[1];
        self.blocks[block_id].z = position[2];
        
        self.regen(font, block_id);
    }

    //pub fn set_text(&mut self,
    pub fn set_text(&mut self, font: &::util::font::TextureFont,
                                    block_id: usize, text: &str) {
        let start = self.blocks[block_id].start;
        let capacity = self.blocks[block_id].capacity;
        let end = start + capacity;
        if text.chars().count() >= capacity {
            panic!("text won't fit into capacity for this TextBlock");
        }
        for i in start..end { // set alpha to zero first
            self.points.buf[0].array_buffer[[i, 5]] = 0.0;
        }
        self.blocks[block_id].text = text.to_string();
        self.regen(font, block_id);
    }

    //pub fn set_rgba(&mut self,
    pub fn set_rgba(&mut self, font: &::util::font::TextureFont,
                                    block_id: usize, rgba: &[f32; 4]) {
        self.blocks[block_id].rgba = *rgba;
        self.regen(font, block_id);
    }

    pub fn set_rot(&mut self, font: &::util::font::TextureFont,
                                    block_id: usize, rot: f32) {
        self.blocks[block_id].rot = rot;
        self.regen(font, block_id);
    }

    pub fn set_char_rot(&mut self, font: &::util::font::TextureFont,
                                    block_id: usize, char_rot: f32) {
        self.blocks[block_id].char_rot = char_rot;
        self.regen(font, block_id);
    }

    pub fn set_spacing(&mut self, font: &::util::font::TextureFont,
                                    block_id: usize, spacing: char) {
        self.blocks[block_id].spacing = spacing;
        self.regen(font, block_id);
    }

    pub fn set_justification(&mut self, font: &::util::font::TextureFont,
                                    block_id: usize, justification: f32) {
        self.blocks[block_id].justification = justification;
        self.regen(font, block_id);
    }

    pub fn set_size(&mut self, font: &::util::font::TextureFont,
                                    block_id: usize, size: f32) {
        self.blocks[block_id].size = size;
        self.regen(font, block_id);
    }

    pub fn set_space(&mut self, font: &::util::font::TextureFont,
                                    block_id: usize, space: f32) {
        self.blocks[block_id].space = space;
        self.regen(font, block_id);
    }


    //fn regen(&mut self, block_id: usize) {
    fn regen(&mut self, font: &::util::font::TextureFont, block_id: usize) {
        //position, rotation etc
        let blk = &self.blocks[block_id]; // alias for brevity
        let const_w = match blk.spacing {
            'M' => 0.0,
            _ => font.size * blk.size * blk.space,
        };
        let vari_w = match blk.spacing {
            'M' => blk.size * blk.space,
            'F' => blk.size,
            _ => 0.0,
        };
        let default = &font.glyph_table[&' '];
        let mut offset = 0.0;
        let mut n_char = 0usize; // used for second loop
        let g_scale = self.point_size / font.height;
        for c in blk.text.chars() {
            let glyph = match &font.glyph_table.get(&c) {
                &Some(g) => g,
                _ => default
            };
            let vi = blk.start + n_char; // index within array_buffer
            self.points.buf[0].array_buffer[[vi, 0]] = offset;
            self.points.buf[0].array_buffer[[vi, 1]] = glyph.verts[2][1] * g_scale * blk.size;
            self.points.buf[0].array_buffer[[vi, 2]] = (blk.z * 10.0).trunc() + blk.size.fract();
            self.points.buf[0].array_buffer[[vi, 3]] = blk.rot + blk.char_rot;
            self.points.buf[0].array_buffer[[vi, 4]] = (blk.rgba[0] * 1000.0).trunc() + blk.rgba[1] * 0.999;
            self.points.buf[0].array_buffer[[vi, 5]] = (blk.rgba[2] * 1000.0).trunc() + blk.rgba[3] * 0.999;
            self.points.buf[0].array_buffer[[vi, 6]] = glyph.x; // uv positions
            self.points.buf[0].array_buffer[[vi, 7]] = glyph.y;
            if blk.spacing == 'F' { //char centre to right
                offset += glyph.w * g_scale * blk.size * 0.5;
            }
            offset += glyph.w * g_scale * vari_w + const_w;
            n_char += 1;
            
        }
        let x_off = blk.justification * offset;
        let sin_r = blk.rot.sin();
        let cos_r = blk.rot.cos();
        for i in 0..n_char {
            let vi = blk.start + i;
            let old_x = self.points.buf[0].array_buffer[[vi, 0]] - x_off;
            let old_y = self.points.buf[0].array_buffer[[vi, 1]];
            self.points.buf[0].array_buffer[[vi, 0]] = blk.x + old_x * cos_r - old_y * sin_r;
            self.points.buf[0].array_buffer[[vi, 1]] = blk.y + old_x * sin_r + old_y * cos_r;
        }
        self.points.buf[0].re_init();
    }
}

pub fn create(font: &::util::font::TextureFont, max_chars: usize, point_size: f32) -> PointText {
    let verts: Vec<f32> = vec![0.0; max_chars * 3];
    let mut new_shape = ::shapes::points::create(&verts, point_size);
    new_shape.buf[0].set_textures(&vec![font.tex.id]);
    new_shape.buf[0].set_blend(true);
    new_shape.unif[[16, 0]] = 0.05; //TODO base on point_size and 
    PointText {
        points: new_shape,
        blocks: vec![],
        max_chars,
        point_size,
    }
}
