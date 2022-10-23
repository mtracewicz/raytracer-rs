use crate::vec3::{unit_vector, Color, Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        &self.origin + &(t * &self.direction)
    }
    pub fn color(&self) -> Color {
        let unit_direction = unit_vector(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        &((1.0 - t)
            * &Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            })
            + &(t * &Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            })
    }
}
