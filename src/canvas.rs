#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Into<u32> for Pixel {
    fn into(self) -> u32 {
        ((self.alpha as u32) << 24)
            | ((self.blue as u32) << 16)
            | ((self.green as u32) << 8)
            | (self.red as u32)
    }
}

impl From<u32> for Pixel {
    fn from(color: u32) -> Self {
        Pixel {
            red: ((color >> 0) & 0xFF) as u8,
            green: ((color >> 8) & 0xFF) as u8,
            blue: ((color >> 16) & 0xFF) as u8,
            alpha: ((color >> 24) & 0xFF) as u8,
        }
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![0u32.into(); width * height],
        }
    }

    #[inline]
    fn get_pixel(&self, x: i32, y: i32) -> Option<&Pixel> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        Some(&self.pixels[y as usize * self.width + x as usize])
    }

    #[inline]
    fn get_pixel_mut(&mut self, x: i32, y: i32) -> Option<&mut Pixel> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        Some(&mut self.pixels[y as usize * self.width + x as usize])
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: usize, height: usize, color: u32) {
        let x_range = x.max(0)..(x + width as i32).min(self.width as i32);
        let y_range = y.max(0)..(y + height as i32).min(self.height as i32);
        for x in x_range {
            for y in y_range.clone() {
                self.get_pixel_mut(x, y).and_then(|pixel| {
                    *pixel = color.into();
                    Some(())
                });
            }
        }
    }

    pub fn fill_circle(&mut self, cx: i32, cy: i32, r: u32, color: u32) {
        let r2 = (r * r) as i32;
        let x_range = (cx - r as i32).max(0)..(cx + r as i32).min(self.width as i32);
        let y_range = (cy - r as i32).max(0)..(cy + r as i32).min(self.height as i32);
        for x in x_range {
            for y in y_range.clone() {
                if (x - cx) * (x - cx) + (y - cy) * (y - cy) <= r2 {
                    self.get_pixel_mut(x, y).and_then(|pixel| {
                        *pixel = color.into();
                        Some(())
                    });
                }
            }
        }
    }
}
