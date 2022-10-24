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
