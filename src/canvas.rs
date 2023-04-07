#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Pixel {
    fn blend_channel(val_1: u8, val_2: u8, alpha: u8) -> u8 {
        let val_1 = val_1 as u32;
        let val_2 = val_2 as u32;
        let alpha = alpha as u32;
        let val = (val_1 * (255 - alpha) + val_2 * alpha) / 255;
        val.min(255) as u8
    }

    pub fn blend(&mut self, other: &Pixel) {
        self.red = Self::blend_channel(self.red, other.red, other.alpha);
        self.green = Self::blend_channel(self.green, other.green, other.alpha);
        self.blue = Self::blend_channel(self.blue, other.blue, other.alpha);
        self.alpha = ((self.alpha as u16) * (255 - other.alpha as u16) + other.alpha as u16) as u8;
    }
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
    pub stride: usize,
    pub pixels: Vec<Pixel>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            stride: width,
            pixels: vec![0u32.into(); width * height],
        }
    }

    #[inline]
    fn get_pixel(&self, x: i32, y: i32) -> Option<&Pixel> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        Some(&self.pixels[y as usize * self.stride + x as usize])
    }

    #[inline]
    fn get_pixel_mut(&mut self, x: i32, y: i32) -> Option<&mut Pixel> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        Some(&mut self.pixels[y as usize * self.stride + x as usize])
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: usize, height: usize, color: u32) {
        let x_range = x.max(0)..(x + width as i32).min(self.width as i32);
        let y_range = y.max(0)..(y + height as i32).min(self.height as i32);
        for x in x_range {
            for y in y_range.clone() {
                self.get_pixel_mut(x, y).and_then(|pixel| {
                    pixel.blend(&color.into());
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
                        pixel.blend(&color.into());
                        Some(())
                    });
                }
            }
        }
    }

    /// 向量叉乘方式计算2倍面积。
    fn double_trangle_area(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> i32 {
        ((x1 - x3) * (y2 - y3) - (x2 - x3) * (y1 - y3)).abs()
    }

    pub fn fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: u32) {
        let lx = x1.min(x2).min(x3).max(0);
        let rx = x1.max(x2).max(x3).min(self.width as i32);
        let ly = y1.min(y2).min(y3).max(0);
        let hy = y1.max(y2).max(y3).min(self.height as i32);
        // 判断原理：点P与三角形ABC的三个顶点构成的三角形的面积S1、S2、S3，如果S1 + S2 + S3 = S，则P在三角形内部。
        // 面积计算方法：
        // 1. 海龙公式：s = (a + b + c) / 2 = √(s(s-a)(s-b)(s-c))
        // 2. 向量叉乘：S = |(x1-x0)(y2-y0) - (x2-x0)(y1-y0)| / 2
        let s = Self::double_trangle_area(x1, y1, x2, y2, x3, y3);
        for y in ly..hy {
            for x in lx..rx {
                let s1 = Self::double_trangle_area(x, y, x2, y2, x3, y3);
                let s2 = Self::double_trangle_area(x1, y1, x, y, x3, y3);
                let s3 = Self::double_trangle_area(x1, y1, x2, y2, x, y);
                if s1 + s2 + s3 == s {
                    self.get_pixel_mut(x, y).and_then(|pixel| {
                        pixel.blend(&color.into());
                        Some(())
                    });
                }
            }
        }

    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2:i32, color: u32) {
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
                pixel.blend(&color.into());
                Some(())
            });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;


    #[test]
    fn area_test() {
        let a = Canvas::double_trangle_area(0, 0, 1, 0, 0, 1);
        assert_eq!(a, 1);
        assert_eq!(Canvas::double_trangle_area(0, 0, 0, 1, 1, 0), 1);
    }
}