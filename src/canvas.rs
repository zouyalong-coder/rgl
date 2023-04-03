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
        let _r2 = (r * r) as i32;
        let x_range = (cx - r as i32).max(0)..(cx + r as i32).min(self.width as i32);
        let y_range = (cy - r as i32).max(0)..(cy + r as i32).min(self.height as i32);
        for x in x_range {
            for y in y_range.clone() {
                if (x - cx) * (x - cx) + (y - cy) * (y - cy) <= _r2 {
                    self.get_pixel_mut(x, y).and_then(|pixel| {
                        *pixel = color.into();
                        Some(())
                    });
                }
            }
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2:i32, color: u32) {
        // y = kx + b
        // y1 = k*x1 + b
        // y2 = k*x2 + b
        // k = (y2 - y1) / (x2 - x1)
        let dx = x2 - x1;
        let dy = y2 - y1;
        if dx == 0 {
            if x1 < 0 || x1 >= self.width as i32 {
                return;
            }
            // make sure y1 < y2
            let (mut y1, mut y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            y1 = y1.max(0);
            y2 = y2.min(self.height as i32);
            for y in y1..y2 {
                self.get_pixel_mut(x1, y).and_then(|pixel| {
                    *pixel = color.into();
                    Some(())
                });
            }
            return;
        }
        let k = dy as f32 / dx as f32;
        let b = y1 as f32 - k * x1 as f32;
        for x in x1..x2 {
            let y = (k * x as f32 + b) as i32;
            let next_y = (k * (x + 1) as f32 + b) as i32;
            // make sure y < next_y
            let (y, next_y) = if y < next_y { (y, next_y) } else { (next_y, y) };
            for cy in y..next_y {
            self.get_pixel_mut(x, cy).and_then(|pixel| {
                *pixel = color.into();
                Some(())
            });
            }
        }
    }
}
