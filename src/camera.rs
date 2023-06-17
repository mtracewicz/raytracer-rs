use crate::{
    helpers::{degrees_to_radians, random_in_unit_disk},
    ray::Ray,
    vec3::{cross_product, unit_vector, Point3, Vec3},
};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        vof: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vof);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let mut camera = Camera {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            lower_left_corner: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            horizontal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            vertical: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            w: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            u: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            v: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            lens_radius: 0.0,
        };

        camera.w = unit_vector(look_from - look_at);
        camera.u = unit_vector(cross_product(v_up, camera.w));
        camera.v = cross_product(camera.w, camera.u);

        camera.origin = look_from;
        camera.horizontal = focus_distance * viewport_width * camera.u;
        camera.vertical = focus_distance * viewport_height * camera.v;
        camera.lower_left_corner = camera.origin
            - camera.horizontal / 2.0
            - camera.vertical / 2.0
            - focus_distance * camera.w;
        camera.lens_radius = aperture / 2.0;
        return camera;
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offset,
        };
    }
}
