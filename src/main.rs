mod vec3d;
mod colour;
mod ray;
mod hittable;
use crate::vec3d::{Vec3D, Colour, Point3D};
use crate::colour::get_colour;
use crate::ray::Ray;
use rayon::prelude::*;
use hittable::{HitRecord, Hittable, HittableList, Sphere};
use std::sync::Arc;


fn ray_colour(r: &Ray, world: &dyn Hittable) -> Colour {

    let mut rec = HitRecord::default();
    if world.hit(r, 0., f32::INFINITY, &mut rec) {
        return Colour::from(0.5 * (rec.normal() + 1.));
    }
    let unit_dir = r.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.);
    let white = Colour::new(1., 1., 1.);
    let blue = Colour::new(0.5, 0.7, 1.);
    (1. - t) * white + t * blue
}

fn main() {
    let aspect_ratio: f32 = 16. / 9.;
    let image_width: usize = 1600;
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let viewport_height: f32 = 2.;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.;

    let origin = Point3D::new(0., 0., 0.);
    let horizontal = Vec3D::new(viewport_width, 0., 0.);
    let vertical = Vec3D::new(0., viewport_height, 0.);
    let separation = Vec3D::new(0., 0., focal_length);
    let lower_left_corner: Point3D = origin - horizontal / 2. - vertical / 2. - separation;

    let world = HittableList::new(vec![
        Arc::new(Sphere::new(Point3D::new(0., -2.5, -1.), 2.)),
        Arc::new(Sphere::new(Point3D::new(0., 0., -1.), 0.5)),
    ]);

    let pixels = (0..image_height).into_par_iter()
        .rev()
        .map(|h| {
            (0..image_width).into_par_iter()
                .map(|w| {
                    let u = (w as f32) / (image_width - 1) as f32;
                    let v = (h as f32) / (image_height - 1) as f32;
                    let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
                    let pixel_colour: Colour = ray_colour(&r, &world);
                    get_colour(pixel_colour)
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("");

    print!("{}", pixels);
    eprintln!("\nDone.");
}
