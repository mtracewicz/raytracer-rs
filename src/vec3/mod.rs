use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.x,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::fmt::Display for &Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

pub fn cross_product(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub fn dot_product(v1: Vec3, v2: Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let l = v.len();
    v / l
}

#[cfg(test)]
mod tests {
    use crate::helpers::assert_approximate_equals;

    use super::*;

    #[test]
    fn test_unit_vector() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let r = unit_vector(v);
        assert_approximate_equals(r.x, 0.267261, 0.000001);
        assert_approximate_equals(r.y, 0.534522, 0.000001);
        assert_approximate_equals(r.z, 0.801784, 0.000001);
    }

    #[test]
    fn test_len() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 2.0,
        };
        assert_approximate_equals(v.len(), 3.0, 0.0001);
    }

    #[test]
    fn f32_multiply_test() {
        let mut v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        const F: f32 = 2.0;
        let mut results = vec![v * F, F * v, v * F, F * v];
        v *= F;
        results.push(v);
        for r in results {
            assert_eq!(r.x, 2.0);
            assert_eq!(r.y, 4.0);
            assert_eq!(r.z, 6.0);
        }
    }

    #[test]
    fn f32_divide_test() {
        let mut v = Vec3 {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        const F: f32 = 2.0;
        let mut results = vec![v / F, v / F];
        v /= F;
        results.push(v);
        for r in results {
            assert_eq!(r.x, 1.0);
            assert_eq!(r.y, 2.0);
            assert_eq!(r.z, 3.0);
        }
    }
}
