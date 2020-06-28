#![feature(clamp)]
mod vec3d;
mod colour;
mod ray;
mod hittable;
mod camera;
use crate::vec3d::{Vec3D, Colour, Point3D};
use crate::colour::get_colour;
use crate::ray::Ray;
use rayon::prelude::*;
use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use std::sync::Arc;
use crate::camera::Camera;

fn ray_colour(r: &Ray, world: &dyn Hittable, depth: u16) -> Colour {

    if depth <= 0 {
        return Colour::new(0., 0., 0.);
    }

    let mut rec = HitRecord::default();
    if world.hit(r, f32::EPSILON, f32::INFINITY, &mut rec) {
        let target: Point3D = rec.p() + rec.normal() + Vec3D::random_in_unit_sphere();
        return 0.5 * ray_colour(&Ray::new(rec.p(), target - rec.p()), world, depth - 1);
    }
    let unit_dir = r.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.);
    let white = Colour::new(1., 1., 1.);
    let blue = Colour::new(0.5, 0.7, 1.);
    (1. - t) * white + t * blue
}

fn main() {
    let aspect_ratio: f32 = 16. / 9.;
    let image_width: usize = 1920;
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel: u16 = 25;
    let max_depth: u16 = 50;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let cam = Camera::new();

    let world = HittableList::new(vec![
        Arc::new(Sphere::new(Point3D::new(0., 0., -1.), 0.5)),
        Arc::new(Sphere::new(Point3D::new(0., -100.5, -1.), 100.)),
    ]);

    let pixels = (0..image_height).into_par_iter()
        .rev()
        .map(|h| {
            (0..image_width).into_par_iter()
                .map(|w| {
                    let pixel_colour = (0..samples_per_pixel).into_par_iter()
                        .map(|_| {
                            let u = (w as f32 + fastrand::f32()) / (image_width - 1) as f32;
                            let v = (h as f32 + fastrand::f32()) / (image_height - 1) as f32;
                            let r: Ray = cam.get_ray(u, v);
                            ray_colour(&r, &world, max_depth)
                        })
                        .collect::<Vec<_>>()
                        .iter()
                        .fold(Colour::new(0., 0., 0.), |acc, x| acc + x);
                    get_colour(pixel_colour, samples_per_pixel as f32)
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("");

    print!("{}", pixels);
    eprintln!("\nDone.");
}
