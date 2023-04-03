use rgl::canvas::Canvas;
use rgl::ppm;

fn main() {
    const width: usize = 200;
    const height: usize = 200;
    let mut cav = Canvas::new(width, height);
    cav.fill_rect(0, 0, 100, 100, 0xff0000ff);
    cav.fill_rect(10, 100, 150, 50, 0xff00ff00);
    cav.fill_circle(100, 50, 50, 0xffffff00);
    cav.draw_line(0, 0, width as i32, height as i32, 0xffff00ff);
    cav.draw_line(0, 0, width as i32 / 3, height as i32, 0xffff00ff);
    cav.draw_line(0, 0, width as i32 , height as i32/3, 0xffff00ff);
    let ppm: ppm::PPM = cav.into();
    ppm.save_to_file("test.ppm").unwrap();
}
