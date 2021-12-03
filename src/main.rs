mod vec3;

use std::fs;
use std::io::Write;
use crate::vec3::Vec3;

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let mut v1 = Vec3::new_i32(1, 2, 3);
    let v2 = Vec3::new(1f64, 2f64, 4f64);
    v1 += -v2;
    v1 *= 3f64;
    v1 /= 5 as f64;
    print!("{} {} {}", v1.x(), v1.y(), v1.z());
    let mut img_file = fs::File::create("test.ppm").expect("create failed");
    img_file.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes()).expect("failed");
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_HEIGHT {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b: f32 = 0.25;


            let r = (r * 256f32) as i32;
            let g = (g * 256f32) as i32;
            let b = (b * 256f32) as i32;
            img_file.write_all(format!("{} {} {}\n", r, g, b).as_bytes()).expect("write failed");
        }
    }
}
