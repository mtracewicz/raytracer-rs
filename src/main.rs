use hittable::{Hittable, Sphere};
use ray::Ray;
use vec3::{Point3, Vec3};

use crate::{
    ppm::{generate_ppm, save_ppm},
    vec3::Color,
};
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

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

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
            let u = (i as f32) / ((image_width - 1) as f32);
            let v = (j as f32) / ((image_height - 1) as f32);
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let index = (image_width * (image_height - j - 1) + i) as usize;
            pixels[index] = r.color(&world);
        }
    }
    let result = generate_ppm(image_width, image_height, &pixels);
    match save_ppm(result.as_str()) {
        Ok(_r) => println!("File saved!"),
        Err(_e) => println!("Error saving the file"),
    }
}
