use std::io::Write;

use crate::canvas;

pub struct PPM {
    pub width: usize,
    pub height: usize,
    pub max_color: usize,
    pub pixels: Vec<canvas::Pixel>,
}

impl PPM {
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(filename)?;
        file.write_all(
            format!(
                "P6\n{width} {height} {max_color}\n",
                width = self.width,
                height = self.height,
                max_color = self.max_color
            )
            .as_bytes(),
        )?;
        // ppm 顺序为按列存储，即先存储第一列的所有像素，再存储第二列的所有像素
        for w in 0..self.width {
            for h in 0..self.height {
                let pixel = self.pixels[w * self.height + h];
                file.write_all(&[pixel.red, pixel.green, pixel.blue])?;
            }
        }
        Ok(())
    }
}

impl From<canvas::Canvas> for PPM {
    fn from(canvas: canvas::Canvas) -> Self {
        PPM {
            width: canvas.width,
            height: canvas.height,
            max_color: 255,
            pixels: canvas.pixels,
        }
    }
}
