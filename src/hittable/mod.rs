use crate::{
    ray::Ray,
    vec3::{dot_product, Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}
impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot_product(ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    fn new(p: Point3, t: f32, ray: &Ray, outward_normal: Vec3) -> HitRecord {
        let mut rec = HitRecord {
            p,
            normal: outward_normal,
            t,
            front_face: false,
        };
        rec.set_face_normal(ray, outward_normal);
        return rec;
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len() * ray.direction.len();
        let half_b = dot_product(oc, ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let squared_discriminant = discriminant.sqrt();
        let mut root = (-half_b - squared_discriminant) / a;
        if root < t_min || t_max < root {
            root = (-half_b + squared_discriminant) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let n = ray.at(root);
        let outward_normal = (n - self.center) / self.radius;
        return Some(HitRecord::new(n, root, &ray, outward_normal));
    }
}

pub fn hit(
    objects: &Vec<Box<dyn Hittable>>,
    ray: &Ray,
    t_min: f32,
    t_max: f32,
) -> Option<HitRecord> {
    let mut hitRecord = HitRecord {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        front_face: false,
    };
    let mut hit_anything = false;
    let mut closest_so_far = t_max;
    for object in objects {
        if let Some(hit) = object.hit(&ray, t_min, closest_so_far) {
            hit_anything = true;
            closest_so_far = hit.t;
            hitRecord = hit;
        }
    }
    if hit_anything {
        return Some(hitRecord);
    }

    None
}