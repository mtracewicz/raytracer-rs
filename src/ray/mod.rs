use crate::{
    hittable::{hit, Hittable},
    vec3::{unit_vector, Color, Point3, Vec3},
};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + (t * self.direction)
    }

    pub fn color(&self, world: &Vec<Box<dyn Hittable>>) -> Color {
        if let Some(hit) = hit(world, self, 0.0, f32::MAX) {
            return 0.5
                * hit.normal
                * Color {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                };
        }
        let unit_direction = unit_vector(self.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)
            * Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + t * Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            }
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::assert_approximate_equals;

    use super::*;

    #[test]
    fn test_color() {
        let r = Ray {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        };
        let res = r.color();
        assert_approximate_equals(res.x, 0.616369, 0.000001);
        assert_approximate_equals(res.y, 0.769822, 0.000001);
        assert_approximate_equals(res.z, 1.0, 0.000001);
    }
}
