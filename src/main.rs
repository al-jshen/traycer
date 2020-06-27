mod vec3d;
mod colour;
mod ray;
use std::io::{stdout, Write};
use crate::vec3d::{Vec3D, Colour, Point3D, unit_vector};
use crate::colour::write_colour;
use crate::ray::Ray;

fn ray_colour(r: &Ray) -> Colour {
    let unit_dir = unit_vector(&r.direction());
    let t = 0.5 * (unit_dir.y() + 1.);
    let white = Colour::new(1., 1., 1.);
    let blue = Colour::new(0.5, 0.7, 1.);
    (1. - t) * white + t * blue
}

fn main() {
    let aspect_ratio: f32 = 16. / 9.;
    let image_width: u16 = 1920;
    let image_height: u16 = (image_width as f32 / aspect_ratio) as u16;

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
        stdout().flush().unwrap();
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
