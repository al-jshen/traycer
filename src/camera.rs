use crate::vec3d::{Vec3D, Point3D};
use crate::ray::Ray;
use std::f32::consts;

pub struct Camera {
    origin: Point3D,
    horizontal: Vec3D,
    vertical: Vec3D,
    lower_left_corner: Point3D,
}

impl Camera {
    pub fn new(origin: Point3D, lookat: Point3D, v_up: Vec3D, vert_fov: f32, aspect_ratio: f32) -> Camera {
        let theta = vert_fov * consts::PI / 180.;
        let h = (theta / 2.).tan();
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - lookat).unit_vector();
        let u = (v_up.cross(&w)).unit_vector();
        let v = w.cross(&u);
        
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2. - vertical/2. - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
