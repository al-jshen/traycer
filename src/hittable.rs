use crate::vec3d::*;
use crate::ray::Ray;

pub struct HitRecord {
    p: Point3D,
    t: f32,
    normal: Vec3D,
    front_face: bool,
}

impl HitRecord {
    pub fn set_normal_face(&mut self, r: &Ray, outward_normal: &Vec3D) {
        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point3D,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f32) -> Sphere {
        Sphere {
            center, 
            radius, 
        }
    }
}

impl Hittable for Sphere {
   fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let sep: Vec3D = r.origin() - self.center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = r.direction().dot(&sep);
        let c: f32 = sep.length_squared() - self.radius * self.radius;
        let discriminant: f32 = half_b * half_b - a * c;
        
        if discriminant > 0. {
            let root: f32 = discriminant.sqrt();
            let temp = if (t_min..t_max).contains(&((-half_b - root) / a)) {
                (-half_b - root) / a
            } else {
                (-half_b + root) / a
            };
            rec.t = temp;
            rec.p = r.at(rec.t);
            let normal = (rec.p - self.center) / self.radius;
            rec.set_normal_face(r, &normal);
            return true;
        }
        false
    }
}
