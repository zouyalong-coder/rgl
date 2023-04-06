use std::io::BufWriter;

use crate::canvas::{Canvas, Pixel};


pub struct Png{
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Png {
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let file = std::fs::File::create(filename)?;
        let ref mut writer = BufWriter::new(file);
        let mut png_encoder = png::Encoder::new(writer, self.width, self.height);
        png_encoder.set_color(png::ColorType::Rgba);
        png_encoder.set_depth(png::BitDepth::Eight);
        let mut writer = png_encoder.write_header()?;
        let mut png_data = Vec::new();
        for pixel in self.pixels.iter() {
            png_data.push(pixel.red);
            png_data.push(pixel.green);
            png_data.push(pixel.blue);
            png_data.push(pixel.alpha);
        }
        writer.write_image_data(&png_data)?;
        Ok(())
    }
}

impl From<Canvas> for Png {
    fn from(value: Canvas) -> Self {
        Self { 
            width: value.width as u32,
            height: value.height as u32, 
            pixels: value.pixels, 
        }
    }
}