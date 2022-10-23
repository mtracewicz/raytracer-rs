use std::{fs::File, io::Write};

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
    let result = generate_ppm(W, H, &arr);
    match save_ppm(result.as_str()) {
        Ok(_r) => println!("File saved!"),
        Err(_e) => println!("Error saving the file"),
    }
}

fn generate_ppm(w: i32, h: i32, arr: &[u8]) -> String {
    let mut result = String::from("P3\n");
    result.push_str(format!("{} {}\n", w, h).as_str());
    result.push_str("255\n");
    for y in 0..h {
        for x in 0..w {
            let start = (3 * w * y + 3 * x) as usize;
            result.push_str(
                format!("{} {} {}\n", arr[start], arr[start + 1], arr[start + 2]).as_str(),
            )
        }
    }
    result
}

fn save_ppm(value: &str) -> Result<(), std::io::Error> {
    let mut file = File::create("image.ppm")?;
    file.write_all(value.as_bytes())?;
    Result::Ok(())
}
