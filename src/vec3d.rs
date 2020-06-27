use std::ops::{Neg, Index, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3D(f32, f32, f32);

impl Vec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3D {
        Vec3D ( x, y, z )
    }
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }
    pub fn length(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn length_squared(&self) -> f32 {
        self.length() * self.length()
    }
    
}

// Various vector functions

pub fn dot(u: &Vec3D, v: &Vec3D) -> f32 {
    (u.0 * v.0) + (u.1 * v.1) + (u.2 * v.2)
}
pub fn cross(u: &Vec3D, v:&Vec3D) -> Vec3D {
    Vec3D (
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    )
}
pub fn unit_vector(u: &Vec3D) -> Vec3D {
   *u / u.length()
}

// - operator

impl Neg for Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Self::Output {
        Vec3D ( self.0, self.1, self.2 )
    }
}

// Vec3D[indexing] 

impl Index<usize> for Vec3D {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index not found in Vec3D.")
        }
    }
}

// Add methods

impl Add for Vec3D {
    type Output = Self;

    fn add(self, v: Self) -> Self {
        Self (
            self.0 + v.0,
            self.1 + v.1,
            self.2 + v.2,
        )
    }
}

impl Add<f32> for Vec3D {
    type Output = Self;

    fn add(self, scalar: f32) -> Self {
        Self (
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar,
        )
    }
}

impl AddAssign for Vec3D {
    fn add_assign(&mut self, v: Self) {
        *self = Self (
            self.0 + v.0,
            self.1 + v.1,
            self.2 + v.2,
        )
    }
}
impl AddAssign<f32> for Vec3D {
    fn add_assign(&mut self, scalar: f32) {
        *self = Self (
            self.0 + scalar,
            self.1 + scalar,
            self.2 + scalar,
        )
    }
}

// Subtract methods
impl Sub for Vec3D {
    type Output = Self;

    fn sub(self, v: Self) -> Self {
        Self (
            self.0 - v.0,
            self.1 - v.1,
            self.2 - v.2,
        )
    }
}

impl Sub<f32> for Vec3D {
    type Output = Self;

    fn sub(self, scalar: f32) -> Self {
        Self (
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar,
        )
    }
}

impl SubAssign for Vec3D {
    fn sub_assign(&mut self, v: Self) {
        *self = Self (
            self.0 - v.0,
            self.1 - v.1,
            self.2 - v.2,
        )
    }
}
impl SubAssign<f32> for Vec3D {
    fn sub_assign(&mut self, scalar: f32) {
        *self = Self (
            self.0 - scalar,
            self.1 - scalar,
            self.2 - scalar,
        )
    }
}

// Multiplication methods 

impl Mul for Vec3D {
    type Output = Self;

    fn mul(self, v: Self) -> Self {
        Self (
            self.0 * v.0,
            self.1 * v.1,
            self.2 * v.2,
        )
    }
}

impl Mul<f32> for Vec3D {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self (
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar,
        )
    }
}

impl MulAssign for Vec3D {
    fn mul_assign(&mut self, v: Self) {
        *self = Self (
            self.0 * v.0,
            self.1 * v.1,
            self.2 * v.2,
        )
    }
}

impl MulAssign<f32> for Vec3D {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self (
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar,
        )
    }
}

// Division methods 

impl Div for Vec3D {
    type Output = Self;

    fn div(self, v: Self) -> Self {
        Self (
            self.0 / v.0,
            self.1 / v.1,
            self.2 / v.2,
        )
    }
}

impl Div<f32> for Vec3D {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self (
            self.0 / scalar,
            self.1 / scalar,
            self.2 / scalar,
        )
    }
}

impl DivAssign for Vec3D {
    fn div_assign(&mut self, v: Self) {
        *self = Self (
            self.0 / v.0,
            self.1 / v.1,
            self.2 / v.2,
        )
    }
}

impl DivAssign<f32> for Vec3D {
    fn div_assign(&mut self, scalar: f32) {
        *self = Self (
            self.0 / scalar,
            self.1 / scalar,
            self.2 / scalar,
        )
    }
}

pub type Point3D = Vec3D;
pub type Colour = Vec3D;
