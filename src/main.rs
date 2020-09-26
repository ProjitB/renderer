use renderer::Point;
use renderer::{draw, save, line};
use image::RgbImage;


fn main() {
    let height = 800;
    let width = 800;
    let mut img = RgbImage::new(height, width);
    
    // draw(&mut img);
    // line(1,2,3,4, img, [256, 0, 0]);
    let red = [255, 0, 0];
    line(1, 2, 30, 40, &mut img, &red);
    save(String::from("image.tga"), img);
}
