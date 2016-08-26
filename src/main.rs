extern crate rust_math;
use rust_math::*;

fn main() {
    let matrix = Matrix4::new(1.0_f32, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0);
    let matrix:[[f32;4];4] = matrix.into();
    println!("{:?}", matrix);

    let pos = Vector3::new(0.0, 0.0, 0.0_f32);
    let dir = Vector3::new(0.0, 0.0, 1.0_f32);
    let up = Vector3::new(0.0, 1.0, 0.0_f32);
    let view = Matrix4::<f32>::look_at(pos, dir, up);
    let view:[[f32;4];4] = view.into();
    println!("{:?}", view);
}
