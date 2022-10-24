pub fn approximate_equals(x: f32, y: f32, d: f32) -> bool {
    y - d <= x && x <= y + d
}
