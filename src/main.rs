mod ppm;

fn main() {
    const W: i32 = 256;
    const H: i32 = 256;
    let mut arr = [0 as u8; (W * H * 3) as usize];
    for j in (0..H).rev() {
        for i in 0..W {
            let r = (i as f32) / (W - 1) as f32;
            let g = (j as f32) / (H - 1) as f32;
            let b = 0.25;
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;
            let start = (3 * W * (255 - j) + 3 * i) as usize;
            arr[start] = ir;
            arr[start + 1] = ig;
            arr[start + 2] = ib;
        }
    }
    let result = ppm::generate_ppm(W, H, &arr);
    match ppm::save_ppm(result.as_str()) {
        Ok(_r) => println!("File saved!"),
        Err(_e) => println!("Error saving the file"),
    }
}
