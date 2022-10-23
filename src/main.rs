use crate::{
    color::Color,
    ppm::{generate_ppm, save_ppm},
};
mod color;
mod ppm;
mod ray;
mod vec3;

fn main() {
    const W: i32 = 256;
    const H: i32 = 256;
    let mut arr = [Color { r: 0, g: 0, b: 0 }; (W * H) as usize];
    for j in (0..H).rev() {
        for i in 0..W {
            let r = (i as f32) / (W - 1) as f32;
            let g = (j as f32) / (H - 1) as f32;
            let b = 0.25;
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;
            let i = (W * (255 - j) + i) as usize;
            arr[i].r = ir;
            arr[i].g = ig;
            arr[i].b = ib;
        }
    }
    let result = generate_ppm(W, H, &arr);
    match save_ppm(result.as_str()) {
        Ok(_r) => println!("File saved!"),
        Err(_e) => println!("Error saving the file"),
    }
}
