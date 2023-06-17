pub mod material {
    use crate::{
        helpers::{random_in_hemisphere, random_in_unit_sphere},
        hittable::HitRecord,
        ray::Ray,
        vec3::{dot_product, reflect, unit_vector, Color},
    };

    pub trait Material {
        fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
            None
        }
    }

    pub struct Lambertian {
        pub albedo: Color,
    }

    impl Material for Lambertian {
        fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
            let mut scatter_direction = hit_record.normal + random_in_hemisphere(hit_record.normal);
            if scatter_direction.near_zero() {
                scatter_direction = hit_record.normal;
            }
            Some((
                self.albedo,
                Ray {
                    origin: hit_record.p,
                    direction: scatter_direction,
                },
            ))
        }
    }

    pub struct Metal {
        pub albedo: Color,
        pub fuzzines: f32,
    }

    impl Material for Metal {
        fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
            let reflected = reflect(&unit_vector(ray.direction), &hit_record.normal);
            let fuzz = if self.fuzzines < 1.0 {
                self.fuzzines
            } else {
                1.0
            };
            let scattered = Ray {
                origin: hit_record.p,
                direction: reflected + fuzz * random_in_unit_sphere(),
            };
            if dot_product(scattered.direction, hit_record.normal) > 0.0 {
                Some((self.albedo, scattered))
            } else {
                None
            }
        }
    }
}
