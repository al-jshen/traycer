mod vec3d;
mod colour;
use std::io::{stdout, Write};
use crate::vec3d::{Vec3D, Colour};
use crate::colour::write_colour;

fn main() {
    let image_width: u16 = 256;
    let image_height: u16 = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rLines remaining: {}", j);
        stdout().flush().unwrap();
        for i in 0..image_width {
            let r = (i as f32) / (image_width - 1) as f32;
            let g = (j as f32) / (image_height - 1) as f32;
            let b = 0.25 as f32;

            let pixel_colour: Colour = Vec3D::new(r, g, b);
            write_colour(pixel_colour);
        }
    }
    eprintln!("\nDone.");
}
