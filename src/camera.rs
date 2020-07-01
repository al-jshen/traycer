use crate::vec3d::{Vec3D, Point3D};
use crate::ray::Ray;
use std::f32::consts;

pub struct Camera {
    origin: Point3D,
    horizontal: Vec3D,
    vertical: Vec3D,
    lower_left_corner: Point3D,
    axes: [Vec3D; 3],
    lens_radius: f32,
}

impl Camera {
    pub fn new(origin: Point3D, lookat: Point3D, v_up: Vec3D, vert_fov: f32, aspect_ratio: f32, aperture: f32) -> Camera {
        let theta = vert_fov * consts::PI / 180.;
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - lookat).unit_vector();
        let u = (v_up.cross(&w)).unit_vector();
        let v = w.cross(&u);
        let axes = [u, v, w];
        
        let focus_dist = (origin - lookat).length();
        let horizontal = viewport_width * u * focus_dist;
        let vertical = viewport_height * v * focus_dist;
        let lower_left_corner = origin - horizontal/2. - vertical/2. - w * focus_dist;

        let lens_radius = aperture / 2.;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            axes,
            lens_radius,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd: Vec3D = self.lens_radius * Vec3D::random_in_unit_disk();
        let offset: Vec3D = self.axes[0] * rd.x() + self.axes[1] * rd.y();
        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset
        )
    }
}
