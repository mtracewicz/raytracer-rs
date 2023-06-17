use crate::vec3::{dot_product, unit_vector, Vec3};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn random_f32() -> f32 {
    rand::random()
}

pub fn random_f32_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * rand::random::<f32>()
}

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    Vec3 {
        x: random_f32_in_range(min, max),
        y: random_f32_in_range(min, max),
        z: random_f32_in_range(min, max),
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = random_vec3(-1.0, 1.0);
        if v.len_squared() >= 1.0 {
            continue;
        }
        break v;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot_product(in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
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
