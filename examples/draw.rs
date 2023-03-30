use rgl::canvas::Canvas;
use rgl::ppm;

fn main() {
    let mut cav = Canvas::new(200, 200);
    cav.fill_rect(0, 0, 100, 100, 0xff0000ff);
    cav.fill_rect(10, 100, 150, 50, 0xff00ff00);
    cav.fill_circle(100, 50, 50, 0xffffff00);
    let ppm: ppm::PPM = cav.into();
    ppm.save_to_file("test.ppm").unwrap();
}
