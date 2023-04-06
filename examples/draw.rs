use rgl::canvas::Canvas;
use rgl::surface::{ppm, png};

fn main() {
    const width: usize = 400;
    const height: usize = 400;
    let mut cav = Canvas::new(width, height);
    cav.fill_rect(0, 0, 100, 100, 0xff0000ff);
    cav.fill_rect(50, 50, 100, 100, 0x9900ff00);
    cav.fill_rect(10, 100, 150, 50, 0xff00ff00);
    cav.fill_circle(100, 50, 50, 0xffffff00);
    cav.draw_line(0, 0, width as i32, height as i32, 0xaaff00ff);
    cav.draw_line(0, 0, width as i32 / 3, height as i32, 0xffff00ff);
    cav.draw_line(0, 0, width as i32 , height as i32/3, 0xffff00ff);
    cav.fill_triangle(10, 10, 50, 50, 100, 60, 0x99ff0000);
    cav.fill_triangle(200, 200, 250, 200, 250, 300, 0xff005555);
    // let ppm: ppm::PPM = cav.into();
    // ppm.save_to_file("test.ppm").unwrap();
    let file: png::Png = cav.into();
    file.save_to_file("test.png").unwrap();
}
