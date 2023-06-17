use std::{
    error,
    sync::{Arc, Mutex},
    thread::available_parallelism,
};

use camera::Camera;
use helpers::random_f32;
use hittable::{Hittable, Sphere};
use material::{Dielectric, Lambertian, Material, Metal};
use vec3::{Color, Point3};

use crate::ppm::{generate_ppm, save_ppm};
mod camera;
mod helpers;
mod hittable;
mod material;
mod ppm;
mod ray;
mod vec3;

fn main() -> Result<(), Box<dyn error::Error>> {
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
    let max_depth = 50;

    let pixels = Arc::new(Mutex::new(vec![
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        (image_width * image_height) as usize
    ]));

    let material_ground: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(Lambertian {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    }));
    let material_center: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(Lambertian {
        albedo: Color {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        },
    }));
    let material_left: Arc<Box<dyn Material + Send + Sync>> =
        Arc::new(Box::new(Dielectric { ir: 1.5 }));
    let material_right: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(Metal {
        albedo: Color {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        fuzzines: 1.0,
    }));

    let world: Arc<Vec<Box<dyn Hittable + Sync + Send>>> = Arc::new(vec![
        Box::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: Arc::clone(&material_center),
        }),
        Box::new(Sphere {
            center: Point3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: Arc::clone(&material_left),
        }),
        Box::new(Sphere {
            center: Point3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: Arc::clone(&material_right),
        }),
        Box::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
            material: Arc::clone(&material_ground),
        }),
    ]);

    let mut handlers = vec![];
    let threads = available_parallelism()?.get() as i32;
    let work_per_thread = image_height / threads;
    for i in 0..image_width {
        for t in 0..threads {
            let world_for_thread = Arc::clone(&world);
            let pixel_for_thread = Arc::clone(&pixels);
            handlers.push(std::thread::spawn(move || {
                for j in t * work_per_thread..(t + 1) * work_per_thread {
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
                        color += &r.color(&world_for_thread, max_depth);
                    }
                    let mut pixels_to_edit = pixel_for_thread.lock().unwrap();
                    pixels_to_edit[index] = color;
                }
            }));
        }

        while handlers.len() > 0 {
            if let Some(handler) = handlers.pop() {
                handler.join().unwrap();
            }
        }
    }
    let pixels_to_use = pixels.lock().unwrap();
    let result = generate_ppm(image_width, image_height, &pixels_to_use, samples_per_pixel);
    match save_ppm(result.as_str()) {
        Ok(_r) => println!("File saved!"),
        Err(_e) => println!("Error saving the file"),
    }
    Ok(())
}
