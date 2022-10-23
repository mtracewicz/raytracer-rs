use std::{fs::File, io::Write};

use crate::vec3::Color;

pub fn generate_ppm(w: i32, h: i32, arr: &[Color]) -> String {
    let mut result = String::from("P3\n");
    result.push_str(format!("{} {}\n", w, h).as_str());
    result.push_str("255\n");
    for y in 0..h {
        for x in 0..w {
            let i = (w * y + x) as usize;
            result.push_str(
                format!("{} {} {}\n", arr[i].x as u8, arr[i].y as u8, arr[i].z as u8).as_str(),
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
