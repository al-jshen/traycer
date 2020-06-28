use crate::vec3d::*;
use crate::ray::Ray;

pub struct HitRecord {
    p: Point3D,
    t: f32,
    normal: Vec3D,
}

impl HitRecord {
    pub fn new(p: Point3D, t: f32, normal: Vec3D) -> HitRecord {
        HitRecord {
            p,
            t, 
            normal,
        }
    }
    pub fn p(&self) -> Point3D {
        self.p
    }
    pub fn t(&self) -> f32 {
        self.t
    }
    pub fn normal(&self) -> Vec3D {
        self.normal
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
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
   fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let sep: Vec3D = r.origin() - self.center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = r.direction().dot(&sep);
        let c: f32 = sep.length_squared() - self.radius * self.radius;
        let discriminant: f32 = half_b * half_b - a * c;
        
        if discriminant > 0. {
            let root: f32 = discriminant.sqrt();
            let t = if (t_min..t_max).contains(&((-half_b - root) / a)) {
                (-half_b - root) / a
            } else {
                (-half_b + root) / a
            };
            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let normal = if r.direction().dot(&outward_normal) < 0. {
                outward_normal
            } else {
                -outward_normal
            };
            Some(HitRecord::new(r.at(t), t, normal));
        }
        None
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { objects }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        for obj in self.objects.iter() {
            if let Some(hit_record) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t();
                return Some(HitRecord::new(hit_record.p(), hit_record.t(), hit_record.normal()));
            }
        }
        None
    }
}
