use std::f32::consts::PI;

use ray_tracer::{tuple::Tuple, canvas::Canvas, color::Color, matrix::Matrix, helper::Axis};

const CANVAS_X: usize = 500;
const CANVAS_Y: usize = 500;

fn main() {
    let mut canvas = Canvas::new(CANVAS_X, CANVAS_Y);
    let middle = Tuple::new_point(250.0, 0.0, 250.0);
    let twelve = Tuple::new_point(0.0, 0.0, 1.0);
    let color = Color::new(1.0, 1.0, 1.0);
    for i in 0..12 {
        let rotated = Matrix::rotation(Axis::Y, i as f32 * (PI / 6f32));
        let point = &rotated * &twelve;
        let final_point = &(point.multiply(150.0)) + &middle;
        let x = final_point.x.round() as usize;
        let y = final_point.z.round() as usize;
        println!("Writing to x: {}, y: {}", x, y);
        canvas.write_pixel(x, y, color);
    }
    canvas.write_to_file("images/chapter_05.ppm");
    println!("File is created"); 
}
