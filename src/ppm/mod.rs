use std::{fs::File, io::Write};

pub fn generate_ppm(w: i32, h: i32, arr: &[u8]) -> String {
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

pub fn save_ppm(value: &str) -> Result<(), std::io::Error> {
    let mut file = File::create("image.ppm")?;
    file.write_all(value.as_bytes())?;
    Result::Ok(())
}
