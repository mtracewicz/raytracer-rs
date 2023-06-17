use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let horizontal = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };
        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        };
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        };
    }
}
