fn approximate_equals(x: f32, y: f32, d: f32) -> bool {
    y - d <= x && x <= y + d
}

pub fn assert_approximate_equals(expected: f32, actual: f32, delta: f32) {
    assert!(
        approximate_equals(expected, actual, delta),
        "Expected: {}, Actual: {}, Delta: {}",
        expected,
        actual,
        delta
    )
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn random_f32() -> f32 {
    rand::random()
}

pub fn random_f32_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * rand::random::<f32>()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    } else {
        return x;
    }
}
