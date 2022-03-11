mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod tools;
mod vec3;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tools::random;
use crate::vec3::{Color, Point3, Vec3};
use std::fs;
use std::fs::File;
use std::io::Write;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    // let t = hit_sphere(Point3::new_i32(0, 0, -1), 0.5, r);
    let (hit, record) = world.hit(r, 0.0, tools::INFINITY);
    if hit {
        return 0.5 * (record.unwrap().normal + Color::new_i32(1, 1, 1));
    }
    let unit_dir = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn write_color(img_file: &mut File, color: Color) {
    let r = (color.x() * 255f64) as i32;
    let g = (color.y() * 255f64) as i32;
    let b = (color.z() * 255f64) as i32;
    img_file
        .write_all(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("write failed");
}

fn main() {
    act();
}

fn act() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let camera = Camera::default();
    // antialiasing sample times
    let sample_times = 10;
    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new_i32(0, 0, -1), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut img_file = fs::File::create("test-antialiasing.ppm").expect("create failed");
    img_file
        .write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .expect("failed");
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut total_color = Color::default();
            for t in 0..sample_times {
                let u = (i as f64 + random()) / (image_width - 1) as f64;
                let v = (j as f64 + random()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                let pixel_color = ray_color(&ray, &world);
                total_color += pixel_color;
            }
            write_color(&mut img_file, total_color / sample_times as f64);
        }
    }
}
