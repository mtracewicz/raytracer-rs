use std::{
    error,
    sync::{Arc, Mutex},
    thread::available_parallelism,
};

use camera::Camera;
use helpers::{random_f32, random_f32_in_range};
use hittable::{Hittable, Sphere};
use material::{Dielectric, Lambertian, Material, Metal};
use vec3::{Color, Point3, Vec3};

use crate::ppm::{generate_ppm, save_ppm};
mod camera;
mod helpers;
mod hittable;
mod material;
mod ppm;
mod ray;
mod vec3;

fn random_scene() -> Arc<Vec<Box<dyn Hittable + Sync + Send>>> {
    let mut world: Vec<Box<dyn Hittable + Sync + Send>> = vec![];

    let ground_material: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(Lambertian {
        albedo: Color {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    }));

    world.push(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: Arc::clone(&ground_material),
    }));

    let material1: Arc<Box<dyn Material + Send + Sync>> =
        Arc::new(Box::new(Dielectric { ir: 1.5 }));
    world.push(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material1,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Point3 {
                x: (a as f32) + 0.9 * random_f32(),
                y: 0.2,
                z: (b as f32) + 0.9 * random_f32(),
            };

            if (center
                - Point3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .len()
                > 0.9
            {
                let material: Box<dyn Material + Send + Sync> = if choose_mat < 0.8 {
                    Box::new(Lambertian {
                        albedo: Color::random(),
                    })
                } else if choose_mat < 0.95 {
                    Box::new(Metal {
                        albedo: Color::random(),
                        fuzzines: random_f32_in_range(0.0, 0.5),
                    })
                } else {
                    Box::new(Dielectric { ir: 1.5 })
                };
                world.push(Box::new(Sphere {
                    center,
                    material: Arc::new(material),
                    radius: 0.2,
                }))
            }
        }
    }

    let material2: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(Lambertian {
        albedo: Color {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        },
    }));
    world.push(Box::new(Sphere {
        center: Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material2,
    }));

    let material3: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(Metal {
        albedo: Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzzines: 0.0,
    }));
    world.push(Box::new(Sphere {
        center: Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material3,
    }));

    return Arc::new(world);
}

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
    let samples_per_pixel = 500;
    let look_from = Point3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let look_at = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    let max_depth = 50;

    let pixels = Arc::new(Mutex::new(vec![
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        (image_width * image_height) as usize
    ]));

    let world: Arc<Vec<Box<dyn Hittable + Sync + Send>>> = random_scene();
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
