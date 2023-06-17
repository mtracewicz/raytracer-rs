use crate::{
    helpers::degrees_to_radians,
    ray::Ray,
    vec3::{cross_product, unit_vector, Point3, Vec3},
};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        vof: f32,
        aspect_ratio: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vof);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross_product(v_up, w));
        let v = cross_product(w, u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
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
