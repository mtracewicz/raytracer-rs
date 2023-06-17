use std::{fs::File, io::Write};

use crate::{helpers::clamp, vec3::Color};

pub fn generate_ppm(w: i32, h: i32, vec: &Vec<Color>, samples_per_pixel: i32) -> String {
    let scale = 1.0 / samples_per_pixel as f32;
    let mut result = String::from("P3\n");
    result.push_str(format!("{} {}\n", w, h).as_str());
    result.push_str("255\n");
    for y in 0..h {
        for x in 0..w {
            let i = (w * y + x) as usize;
            let r = (vec[i].x * scale).sqrt();
            let g = (vec[i].y * scale).sqrt();
            let b = (vec[i].z * scale).sqrt();
            result.push_str(
                format!(
                    "{} {} {}\n",
                    (256.00 * clamp(r, 0.0, 0.99)) as u8,
                    (256.00 * clamp(g, 0.0, 0.99)) as u8,
                    (256.00 * clamp(b, 0.0, 0.99)) as u8
                )
                .as_str(),
            )
        }
    }
    result
}

pub fn save_ppm(value: &str) -> Result<(), std::io::Error> {
    let mut file = File::create("image.ppm")?;
    file.write_all(value.as_bytes())?;
    Result::Ok(())
}
