use crate::{
    helpers::{random_f32, random_in_hemisphere, random_in_unit_sphere},
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot_product, reflect, refract, unit_vector, Color},
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
        let reflected = reflect(unit_vector(ray.direction), hit_record.normal);
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

pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(ray.direction);
        let cos_theta = dot_product(-unit_direction, hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f32()
        {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        Some((
            attenuation,
            Ray {
                origin: hit_record.p,
                direction: direction,
            },
        ))
    }
}
