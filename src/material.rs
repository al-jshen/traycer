use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3d::{Colour, Vec3D};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Colour },
    Metal { albedo: Colour, fuzziness: f32 },
    Dielectric { refr_index: f32 },
}

impl Material {

    pub fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<(Ray, Colour)> {
        match self {
            Material::Lambertian {albedo} => {
                let scatter_direction: Vec3D = rec.normal() + Vec3D::random_unit_vector();
                Some((Ray::new(rec.p(), scatter_direction), *albedo))
            }
            Material::Metal {albedo, fuzziness} => {
                let reflected = r_in.direction().unit_vector().reflect(&rec.normal());
                let scattered = Ray::new(rec.p(), reflected + *fuzziness * Vec3D::random_in_unit_sphere());
                if scattered.direction().dot(&rec.normal()) > 0. {
                    return Some((scattered, *albedo))
                } else {
                    None
                }
            }
            Material::Dielectric {refr_index} => {
                let attenuation = Colour::new(1., 1., 1.);
                let refr_index_ratio = if rec.front_face() {
                    1. / refr_index
                } else {
                    *refr_index
                };
                let unit_dir = r_in.direction().unit_vector();
                assert!(unit_dir.dot(&rec.normal()) < 0.);

                let cos_theta = -unit_dir.dot(&rec.normal());
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();
                if refr_index_ratio * sin_theta > 1. { 
                    let reflected = unit_dir.reflect(&rec.normal());
                    return Some((Ray::new(rec.p(), reflected), attenuation));
                } 
                let reflect_prob = shlick(cos_theta, refr_index_ratio);
                if fastrand::f32() < reflect_prob {
                    let reflected = unit_dir.reflect(&rec.normal());
                    return Some((Ray::new(rec.p(), reflected), attenuation));
                }
                let refracted = unit_dir.refract(&rec.normal(), refr_index_ratio);
                Some((Ray::new(rec.p(), refracted), attenuation))
            }
        }
    }
}

fn shlick(cosine: f32, refr_index: f32) -> f32 {
    let r0 = ((1. - refr_index) / (1. + refr_index)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
