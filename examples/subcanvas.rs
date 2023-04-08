use rgl::canvas::Canvas;
use rgl::surface::png;

fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;
    let mut master_canvas = Canvas::new(WIDTH, HEIGHT);
    master_canvas.fill(0xffffffff);
    let mut left_top = master_canvas.sub_canvas(0, 0, WIDTH as u32/2, HEIGHT as u32/2).unwrap();
    left_top.fill_circle(150, 150, (WIDTH as u32)/4, 0xffff0000);
    let mut right_top = master_canvas.sub_canvas(WIDTH as i32/2, 0, WIDTH as u32/2, HEIGHT as u32/2).unwrap();
    right_top.fill_triangle(0, 0, -50, 50, 50, 70, 0x6600ff00);
    let file: png::Png = master_canvas.into();
    file.save_to_file("test.png").unwrap();
}