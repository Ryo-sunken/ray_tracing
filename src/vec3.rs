use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Uniform};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Vector3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Vector3 {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub(crate) fn one() -> Self {
        Self {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }

    pub(crate) fn unit_x() -> Self {
        Self {
            x: 1.,
            y: 0.,
            z: 0.,
        }
    }

    pub(crate) fn unit_y() -> Self {
        Self {
            x: 0.,
            y: 1.,
            z: 0.,
        }
    }

    pub(crate) fn unit_z() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 1.,
        }
    }

    pub(crate) fn randu(dist: &Uniform<f64>, engine: &mut ChaCha8Rng) -> Self {
        Self {
            x: dist.sample(engine),
            y: dist.sample(engine),
            z: dist.sample(engine),
        }
    }

    pub(crate) fn random_unit_vector(engine: &mut ChaCha8Rng) -> Self {
        let a = Uniform::new(0., 2. * std::f64::consts::PI).sample(engine);
        let z = Uniform::new(-1., 1.).sample(engine);
        let r = ((1_f64 - z * z)).sqrt();
        Self::new(r * a.cos(), r * a.sin(), z)
    }

    pub(crate) fn random_in_unit_sphere(engine: &mut ChaCha8Rng) -> Self {
        let dist = Uniform::new(-1., 1.);
        loop {
            let p = Self::randu(&dist, engine);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub(crate) fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub(crate) fn normalized(self) -> Self {
        self / self.length()
    }

    pub(crate) fn dot(self, rhs: Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub(crate) fn cross(self, rhs: Vector3) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
