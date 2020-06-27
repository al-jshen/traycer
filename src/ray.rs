use crate::vec3d::{Point3D, Vec3D};

#[derive(Debug)]
pub struct Ray {
    origin: Point3D,
    direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3D) -> Ray {
        Ray {
            origin,
            direction,
        }
    }
    pub fn origin(&self) -> Point3D {
        self.origin
    }
    pub fn direction(&self) -> Vec3D {
        self.direction
    }
    pub fn at(&self, t: f32) -> Point3D {
        self.origin + self.direction * t
    }
}
