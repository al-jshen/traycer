//use std::ops::{Neg, Index, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::ops;
use impl_ops::*;
use std::f32::consts;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3D(f32, f32, f32);

impl Vec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3D {
        Vec3D ( x, y, z )
    }
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn dot(&self, other: &Vec3D) -> f32 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }
    pub fn cross(&self, other: &Vec3D) -> Vec3D {
        Vec3D (
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
    pub fn unit_vector(&self) -> Vec3D {
        *self / self.length()
    }
    pub fn random(min: f32, max: f32) -> Vec3D {
        Vec3D (
            (max - min) * fastrand::f32() + min,
            (max - min) * fastrand::f32() + min,
            (max - min) * fastrand::f32() + min,
        )
    }
    pub fn random_in_unit_sphere() -> Vec3D {
        loop {
            let temp = Vec3D::random(-1., 1.);
            if temp.length_squared() < 1. {
                return temp;
            }
        }
    }
    pub fn random_unit_vector() -> Vec3D {
        let theta = fastrand::f32() * consts::PI;
        let z = (1. - -1.) * fastrand::f32() + -1.;
        let r = (1. - z * z).sqrt();
        Vec3D (r * theta.cos(), r * theta.sin(), z)
    }
    pub fn random_in_hemisphere(normal: &Vec3D) -> Vec3D {
        let in_unit_sphere = Vec3D::random_in_unit_sphere();
        // positive dot product = same hemisphere/direction as normal
        in_unit_sphere.dot(normal).signum() * in_unit_sphere
    }
    pub fn reflect(&self, normal: &Vec3D) -> Vec3D {
        self - 2. * self.dot(normal) * normal
    }
    pub fn refract(&self, normal: &Vec3D, refr_index_ratio: f32) -> Vec3D {
        let cos_theta: f32 = (-self).dot(normal);
        let r_out_parallel: Vec3D = refr_index_ratio * (self + cos_theta * normal);
        let r_out_perp: Vec3D = - (1. - r_out_parallel.length_squared()).sqrt() * normal;
        r_out_parallel + r_out_perp
    }
}

// - operator
impl ops::Neg for Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Self::Output {
        Vec3D ( self.0, self.1, self.2 )
    }
}

impl ops::Neg for &Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Self::Output {
        Vec3D ( self.0, self.1, self.2 )
    }
}

// Vec3D[indexing] 

impl ops::Index<usize> for Vec3D {
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

impl_op_ex!(+ |u: &Vec3D, v: &Vec3D| -> Vec3D { Vec3D(u.0 + v.0, u.1 + v.1, u.2 + v.2)});
// impl Add for Vec3D {
//     type Output = Self;
// 
//     fn add(self, v: Self) -> Self {
//         Self (
//             self.0 + v.0,
//             self.1 + v.1,
//             self.2 + v.2,
//         )
//     }
// }

impl_op_ex_commutative!(+ |u: &Vec3D, t: f32| -> Vec3D { Vec3D(u.0 + t, u.1 + t, u.2 + t) });
// impl Add<f32> for Vec3D {
//     type Output = Self;
// 
//     fn add(self, scalar: f32) -> Self {
//         Self (
//             self.0 * scalar,
//             self.1 * scalar,
//             self.2 * scalar,
//         )
//     }
// }
 
impl_op_ex!(+= |u: &mut Vec3D, v: &Vec3D| { u.0 += v.0; u.1 += v.1; u.2 += v.2;});
// impl ops::AddAssign for Vec3D {
//     fn add_assign(&mut self, v: Self) {
//         *self = Self (
//             self.0 + v.0,
//             self.1 + v.1,
//             self.2 + v.2,
//         )
//     }
// }
impl_op_ex!(+= |u: &mut Vec3D, t: f32| { u.0 += t; u.1 += t; u.2 += t;});
// impl ops::AddAssign<f32> for Vec3D {
//     fn add_assign(&mut self, scalar: f32) {
//         *self = Self (
//             self.0 + scalar,
//             self.1 + scalar,
//             self.2 + scalar,
//         )
//     }
// }

// Subtract methods
impl_op_ex!(- |u: &Vec3D, v: &Vec3D| -> Vec3D { Vec3D(u.0 - v.0, u.1 - v.1, u.2 - v.2)});
impl_op_ex_commutative!(- |u: &Vec3D, t: f32| -> Vec3D { Vec3D(u.0 - t, u.1 - t, u.2 - t) });
impl_op_ex!(-= |u: &mut Vec3D, v: &Vec3D| { u.0 -= v.0; u.1 -= v.1; u.2 -= v.2;});
impl_op_ex!(-= |u: &mut Vec3D, t: f32| { u.0 -= t; u.1 -= t; u.2 -= t;});

// Multiplication methods 

impl_op_ex!(* |u: &Vec3D, v: &Vec3D| -> Vec3D { Vec3D(u.0 * v.0, u.1 * v.1, u.2 * v.2)});
impl_op_ex_commutative!(* |u: &Vec3D, t: f32| -> Vec3D { Vec3D(u.0 * t, u.1 * t, u.2 * t) });
impl_op_ex!(*= |u: &mut Vec3D, v: &Vec3D| { u.0 *= v.0; u.1 *= v.1; u.2 *= v.2;});
impl_op_ex!(*= |u: &mut Vec3D, t: f32| { u.0 *= t; u.1 *= t; u.2 *= t;});


// Division methods 

impl_op_ex!(/ |u: &Vec3D, v: &Vec3D| -> Vec3D { Vec3D(u.0 / v.0, u.1 / v.1, u.2 / v.2)});
impl_op_ex_commutative!(/ |u: &Vec3D, t: f32| -> Vec3D { Vec3D(u.0 / t, u.1 / t, u.2 / t) });
impl_op_ex!(/= |u: &mut Vec3D, v: &Vec3D| { u.0 /= v.0; u.1 /= v.1; u.2 /= v.2;});
impl_op_ex!(/= |u: &mut Vec3D, t: f32| { u.0 /= t; u.1 /= t; u.2 /= t;});

pub type Point3D = Vec3D;
pub type Colour = Vec3D;
