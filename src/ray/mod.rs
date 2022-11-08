use crate::vec3::{dot_product, unit_vector, Color, Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        &self.origin + &(t * &self.direction)
    }

    pub fn color(&self) -> Color {
        let t1 = self.hit_sphere(
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
        );
        if t1 > 0.0 {
            let n = unit_vector(
                &(self.at(t1)
                    - &Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0,
                    }),
            );
            return 0.5
                * Color {
                    x: n.x + 1.0,
                    y: n.y + 1.0,
                    z: n.z + 1.0,
                };
        }
        let unit_direction = unit_vector(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)
            * Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + t * &Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            }
    }

    fn hit_sphere(&self, center: &Point3, radius: f32) -> f32 {
        let oc = self.origin - center;
        let a = dot_product(&self.direction, &self.direction);
        let b = 2.0 * dot_product(&oc, &self.direction);
        let c = dot_product(&oc, &oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a);
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
