extern crate rand;

use rand::Rng;
use std::ops;

pub fn sqr(x: f32) -> f32 {
    x * x
}

pub fn clamp<T>(x: T, min: T, max: T) -> T
where T : PartialOrd {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn to_linear(srgb_col: f32) -> f32 {
    if srgb_col <= 0.04045 {
        srgb_col / 12.92
    } else {
        ((srgb_col + 0.055) / 1.055).powf(2.4)
    }
}

fn to_srgb(linear_col: f32) -> f32 {
    if linear_col <= 0.0031308 {
        linear_col * 12.92
    } else {
        1.055 * linear_col.powf(1.0 / 2.4) - 0.055
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    v: [f32; 3]
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 {
            v: [0.0, 0.0, 0.0]
        }
    }

    pub fn one() -> Vec3 {
        Vec3 {
            v: [1.0, 1.0, 1.0]
        }
    }

    pub fn up() -> Vec3 {
        Vec3 { v: [0.0, 1.0, 0.0] }
    }
    
    pub fn forward() -> Vec3 {
        Vec3 { v: [0.0, 0.0, -1.0] }
    }

    pub fn right() -> Vec3 {
        Vec3 { v: [1.0, 0.0, 0.0] }
    }

    pub fn new(x: f32, y:f32, z:f32) -> Vec3 {
        Vec3 {
            v: [x, y, z]
        }
    }

    pub fn new_fill(value: f32) -> Vec3 {
        Vec3 {
            v: [value, value, value]
        }
    }

    pub fn v(&self) -> &[f32; 3] {
        &self.v
    }

    pub fn x(&self) -> f32 {
        self.v[0]
    }
    pub fn y(&self) -> f32 {
        self.v[1]
    }
    pub fn z(&self) -> f32 {
        self.v[2]
    }

    pub fn r(&self) -> f32 {
        self.v[0]
    }
    pub fn g(&self) -> f32 {
        self.v[1]
    }
    pub fn b(&self) -> f32 {
        self.v[2]
    }

    pub fn set(&mut self, idx: usize, value: f32) {
        self.v[idx] = value
    }

    pub fn length_sqr(&self) -> f32 {
        (self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2])
    }

    pub fn length(&self) -> f32 {
        self.length_sqr().sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        self.v[0] /= len;
        self.v[1] /= len;
        self.v[2] /= len;
    }

    pub fn normalized(&self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[1] * other.v[2] - self.v[2] * other.v[1],
                self.v[2] * other.v[0] - self.v[0] * other.v[2],
                self.v[0] * other.v[1] - self.v[1] * other.v[0]
            ]
        }
    }

    pub fn to_srgb(&self) -> Vec3 {
        Vec3::new(to_srgb(self.r()), to_srgb(self.g()), to_srgb(self.b()))
    }

    pub fn to_linear(&self) -> Vec3 {
        Vec3::new(to_linear(self.r()), to_linear(self.g()), to_linear(self.b()))
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - &(2.0 * self.dot(normal) * normal)
    }

    pub fn refract(&self, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let uv = self.normalized();
        let dt = uv.dot(normal);
        let discriminant = 1.0 - sqr(ni_over_nt) * (1.0 - sqr(dt));
        if discriminant > 0.0 {
            Some(ni_over_nt * (uv - normal * dt) - normal * discriminant.sqrt())
        } else {
            None
        }
    }
}

impl<'a> ops::Add<&Vec3> for &'a Vec3 {
    type Output = Vec3;
    
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]
            ]
        }
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]
            ]
        }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]
            ]
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]
            ]
        }
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        *self = Self {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]
            ]
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            v: [
                -self.v[0],
                -self.v[1],
                -self.v[2]
            ]
        }
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            v: [
                -self.v[0],
                -self.v[1],
                -self.v[2]
            ]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        &self + &(-other)
    }
}

impl<'a, 'b> ops::Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'a Vec3) -> Vec3 {
        self + &(-other)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            v: [self.v[0] * scalar,
                self.v[1] * scalar,
                self.v[2] * scalar
            ]
        }
    }
}

impl ops::Mul<f32> for &Vec3  {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            v: [self.v[0] * scalar,
                self.v[1] * scalar,
                self.v[2] * scalar
            ]
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;
    
    fn mul(self, v: &Vec3) -> Vec3 {
        v * self
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] * v.v[0],
                self.v[1] * v.v[1],
                self.v[2] * v.v[2]
            ]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self {
            v: [self.v[0] / scalar,
                self.v[1] / scalar,
                self.v[2] / scalar
            ]
        }
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;
    
    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] / scalar,
                self.v[1] / scalar,
                self.v[2] / scalar
            ]
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        *self = Self {
            v: [self.v[0] / scalar,
                self.v[1] / scalar,
                self.v[2] / scalar
            ]
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3::one();
        if p.length_sqr() < 1.0 {
            return p;
        }
    }
}

/// Returns a random point in unit circle with center in origin
pub fn random_in_unit_circle() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.length_sqr() < 1.0 {
            return p;
        }
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = sqr((1.0 - ref_idx) / (1.0 + ref_idx));
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
