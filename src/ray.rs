use crate::vec3d::{Point3D, Vec3D};

#[derive(Debug)]
pub struct Ray {
    origin: Point3D,
    direction: Vec3D,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3D, time: f32,) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }
    pub fn origin(&self) -> Point3D {
        self.origin
    }
    pub fn direction(&self) -> Vec3D {
        self.direction
    }
    pub fn time(&self) -> f32 {
        self.time
    }
    pub fn at(&self, t: f32) -> Point3D {
        self.origin + self.direction * t
    }
}
