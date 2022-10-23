use ray::Ray;
use vec3::{Point3, Vec3};

use crate::{
    ppm::{generate_ppm, save_ppm},
    vec3::Color,
};
mod ppm;
mod ray;
mod vec3;

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
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
        x: viewport_height,
        y: 0.0,
        z: 0.0,
    };
    let lower_left_corner = &(&origin - &(&horizontal / 2.0))
        - &(&(&vertical / 2.0)
            - &Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            });

    let mut vec: Vec<Vec3> = vec![
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        (image_width * image_height) as usize
    ];
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = (i as f32) / ((image_width - 1) as f32);
            let v = (j as f32) / ((image_height - 1) as f32);
            let r = Ray {
                origin: origin,
                direction: &(&lower_left_corner + &(u * &horizontal))
                    + &(&(v * &vertical) - &origin),
            };

            let i = (image_width * (image_height - 1 - j) + i) as usize;
            vec[i] = r.color();
        }
    }
    let result = generate_ppm(image_width, image_height, &vec);
    match save_ppm(result.as_str()) {
        Ok(_r) => println!("File saved!"),
        Err(_e) => println!("Error saving the file"),
    }
}
