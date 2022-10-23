fn main() {
    println!("P3");
    println!("256 256");
    println!("255");
    let w = 256;
    let h = 256;
    for j in (0..h).rev() {
        for i in 0..w {
            let r = (i as f32) / (w - 1) as f32;
            let g = (j as f32) / (h - 1) as f32;
            let b = 0.25;
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
