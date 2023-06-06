use camera::Camera;
use helpers::random_f32;
use hittable::{Hittable, Sphere};
use vec3::{Point3, Vec3};

use crate::{
    ppm::{generate_ppm, save_ppm},
    vec3::Color,
};
mod camera;
mod helpers;
mod hittable;
mod ppm;
mod ray;
mod vec3;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let image_width = if args.len() >= 2 {
        match args[1].parse::<i32>() {
            Ok(i) => i,
            Err(_i) => 400,
        }
    } else {
        400
    };
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_height = ((image_width as f32) / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let camera = Camera::new(aspect_ratio);

    let mut pixels: Vec<Vec3> = vec![
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        (image_width * image_height) as usize
    ];

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
        }),
    ];

    for i in 0..image_width {
        for j in 0..image_height {
            let mut color = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let index = (image_width * (image_height - j - 1) + i) as usize;
            for _ in 0..samples_per_pixel {
                let u = ((i as f32) + random_f32()) / ((image_width - 1) as f32);
                let v = ((j as f32) + random_f32()) / ((image_height - 1) as f32);
                let r = camera.get_ray(u, v);
                color += &r.color(&world);
            }
            pixels[index] = color;
        }
    }
    let result = generate_ppm(image_width, image_height, &pixels, samples_per_pixel);
    match save_ppm(result.as_str()) {
        Ok(_r) => println!("File saved!"),
        Err(_e) => println!("Error saving the file"),
    }
}
