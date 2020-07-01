use crate::vec3d::*;
use crate::ray::Ray;
use std::sync::Arc;
use crate::material::Material;

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    p: Point3D,
    t: f32,
    normal: Vec3D,
    front_face: bool,
}

impl HitRecord {
    pub fn set_normal_face(&mut self, r: &Ray, outward_normal: &Vec3D) {
        self.front_face = r.direction().dot(outward_normal) < 0.;
        //eprintln!("{:?} {:?} {}", r.direction(), outward_normal, r.direction().dot(outward_normal));
        if self.front_face() {
            assert!(outward_normal.dot(&r.direction()) < 0.);
            self.normal = *outward_normal;
            //eprintln!("{:?}", self.normal);
        } else {
            assert!(-outward_normal.dot(&r.direction()) < 0.);
            self.normal = -*outward_normal;
            //eprintln!("{:?}", self.normal);
        }
        //self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
        assert!(self.normal().dot(&r.direction()) <= 0.);
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
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> Option<Material>;
}

pub struct Sphere {
    center: Point3D,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f32, material: Material) -> Sphere {
        Sphere {
            center, 
            radius, 
            material,
        }
    }
}

impl Hittable for Sphere {
   fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> Option<Material> {
        let sep: Vec3D = r.origin() - self.center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = r.direction().dot(&sep);
        let c: f32 = sep.length_squared() - self.radius * self.radius;
        let discriminant: f32 = half_b * half_b - a * c;
        
        if discriminant > 0. {
            let root: f32 = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3D = (rec.p - self.center) / self.radius;
                rec.set_normal_face(r, &outward_normal);
                return Some(self.material);
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3D = (rec.p - self.center) / self.radius;
                rec.set_normal_face(r, &outward_normal);
                return Some(self.material);
            }
        }
        None
    }
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> HittableList {
        HittableList { objects }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> Option<Material> {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if let Some(material) = obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = Some(material);
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
