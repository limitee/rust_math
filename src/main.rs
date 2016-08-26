extern crate rust_math;
use rust_math::*;

fn main() {
    let matrix = Matrix4::new(1.0_f32, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0);
    let matrix:[[f32;4];4] = matrix.into();
    println!("{:?}", matrix);
}
