use image::{RgbImage, Rgb};

#[derive(Debug)]
pub struct Point{
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    pub fn new(x: i32, y:i32, z:i32) -> Result<Point, &'static str> {
        Ok(Point{x,y,z})
    }
}

pub fn save(filename: String, img: RgbImage) {
    img.save(filename).unwrap()
}


pub fn draw(img: &mut RgbImage) {
    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
        }
    }
}


fn swap(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t;
}

pub fn make_line<F: FnMut(u32, u32)>(x0: u32, x1: u32, y0: u32, y1: u32, dx: i32, dy: i32, mut imgput: F) {
    let (lx, ly, rx, ry) = if x0 > x1 { // Going left to right
        (x1, y1, x0, y0) } else {(x0, y0, x1, y1)};

    let mut y = ly; // compiler will anyway replace y with y0
    let mut error = 0;
    let numerator_error_increment = 2 *dy;
    println!("({},{}),({},{})", lx, ly, rx, ry);
    for x in lx..rx {
        imgput(x,y);
        error += numerator_error_increment;
        if error > dx {
            match ry>ly{
                true => y+=1,
                false => y-=1,
            }
            error -= 2*dx;
        }
    }
}

pub fn line(x0: u32, y0: u32, x1: u32, y1: u32, img: &mut RgbImage, color: &[u8; 3]) {
    let dx = if x0>x1 {x0-x1} else {x1-x0} as i32;
    let dy = if y0>y1 {y0-y1} else {y1-y0} as i32;
    if dx < dy { // rotating if above 45 degrees
        make_line(y0, y1, x0, x1, dy, dx, |x: u32, y: u32| {img.put_pixel(y, x, Rgb(*color))});
    } else {
        make_line(y0, y1, x0, x1, dy, dx, |x: u32, y: u32| {img.put_pixel(x, y, Rgb(*color))});
    };
}
    //(200, 12) (300, 400)


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let x = Point::new(3,4,5).unwrap();
        assert_eq!("hello world", "hello world")
    }

    #[test]
    fn line_compilation() {
        let mut img = RgbImage::new(800, 800);
        let green = [0, 255, 0];
        let red = [255, 0, 0];
        let blue = [0, 0, 255];
        // line(1, 1, 300, 400, &mut img, &red);
        line(200, 12, 300, 400, &mut img, &green);
        line(300, 400, 200, 12, &mut img, &red);
        save(String::from("test1.tga"), img);
    }
    
    #[test]
    fn write_over_line_blue() {
        let mut img = RgbImage::new(800, 800);
        let green = [0, 255, 0];
        let red = [255, 0, 0];
        let blue = [0, 0, 255];
        line(200, 12, 300, 400, &mut img, &green);
        line(300, 400, 200, 12, &mut img, &blue);
        save(String::from("test2.tga"), img);
    }


    #[test]
    fn swap_test() {
        let mut x = 3;
        let mut y = 4;
        swap(&mut x, &mut y);
        assert_eq!(x, 4);
    }
}
