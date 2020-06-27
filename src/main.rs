mod vec3d;
mod colour;
mod ray;
use crate::vec3d::{Vec3D, Colour, Point3D};
use crate::colour::write_colour;
use crate::ray::Ray;
// use rayon::prelude::*;

fn hit_sphere(center: &Point3D, radius: f32, r: &Ray) -> bool {
    // t: ray scalar. want to solve for this.
    // B: ray direction.
    // A: ray origin.
    // C: sphere origin. 
    // r: sphere radius.
    //
    // ray hits the sphere if there exists a solution to
    // t^2 B.B + 2t * B.(A-C) + (A-C).(A-C) - r^2 = 0
    // condition met if discriminant b^2 - 4ac > 0
    //
    // a = B.B
    // b = 2*B.(A-C)
    // c = (A-C).(A-C) - r^2
    let sep: Vec3D = r.origin() - center;
    let a = r.direction().dot(&r.direction());
    let b = 2. * r.direction().dot(&sep);
    let c = sep.dot(&sep) - radius * radius;
    b * b - 4. * a * c > 0.
}

fn ray_colour(r: &Ray) -> Colour {
    if hit_sphere(&Point3D::new(0., 0., -1.), 0.5, r) {
        return Colour::new(1., 0., 0.);
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

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let viewport_height: f32 = 2.;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.;

    let origin = Point3D::new(0., 0., 0.);
    let horizontal = Vec3D::new(viewport_width, 0., 0.);
    let vertical = Vec3D::new(0., viewport_height, 0.);
    let separation = Vec3D::new(0., 0., focal_length);
    let lower_left_corner: Point3D = origin - horizontal / 2. - vertical / 2. - separation;


    for j in (0..image_height).rev() {
        eprintln!("\rLines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f32) / (image_width - 1) as f32;
            let v = (j as f32) / (image_height - 1) as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_colour: Colour = ray_colour(&r);
            write_colour(pixel_colour);
        }
    }
    eprintln!("\nDone.");
}
