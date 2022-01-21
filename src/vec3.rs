use rand::prelude::*;

use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vec3(f32, f32, f32);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new() -> Self {
        Vec3(0., 0., 0.)
    }

    pub fn from(x: f32, y: f32, z: f32) -> Self {   
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::from(rng.gen_range(-1.0..1.),
                               rng.gen_range(-1.0..1.),
                               0.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = Vec3::from(rng.gen_range(-1.0..1.),
                               rng.gen_range(-1.0..1.),
                               rng.gen_range(-1.0..1.));
            if v.length_squared() < 1. {
                return v;
            }
        }

    }
    
    pub fn random_unit_vector() -> Self {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn near_zero(&self) -> bool {
        let min = 1e-8;

        self.0.abs() < min && self.1.abs() < min && self.2.abs() < min
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self; 

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, n: f32) -> Self::Output {
        Vec3(self.0 * n, self.1 * n, self.2 * n)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3(self * v.0, self * v.1, self * v.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, den: f32) -> Self::Output {
        self * (1./den)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 *= 1./rhs;
        self.1 *= 1./rhs;
        self.2 *= 1./rhs;
    }
}
    
pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3(
        v1.1 * v2.2 - v1.2 * v2.1,
        v1.2 * v2.0 - v1.0 * v2.2,
        v1.0 * v2.1 - v1.1 * v2.0
    )
}

pub fn reflect(v1: &Vec3, v2: &Vec3) -> Vec3 {
    *v1 - 2. * dot(v1, v2) * *v2
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(&-uv, &n).min(1.);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_para = -(((1. - r_out_perp.length_squared()).abs()).sqrt()) * n;
    r_out_perp + r_out_para
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let len = v.length();
    v / len
}
