use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3d::{Colour, Vec3D};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Colour },
    Metal { albedo: Colour, fuzziness: f32 }
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
        }
    }
}
