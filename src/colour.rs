use crate::vec3d::Colour;

pub fn get_colour(pixel_colour: Colour, samples_per_pixel: f32) -> String {
    let r = transform_colour(pixel_colour.x(), samples_per_pixel);
    let g = transform_colour(pixel_colour.y(), samples_per_pixel);
    let b = transform_colour(pixel_colour.z(), samples_per_pixel);

    format!("{} {} {}\n", r, g, b) 
}

fn transform_colour(init_val: f32, samples_per_pixel: f32) -> u16 {
    let mut new_val = init_val / samples_per_pixel;
    new_val = new_val.powf(1./2.2); // gamma correction
    new_val = new_val.clamp(0., 0.999) * 256.;
    new_val as u16
}
