use crate::vec3d::Colour;

pub fn get_colour(pixel_colour: Colour, samples_per_pixel: f32) -> String {
    let r = (pixel_colour.x() / samples_per_pixel).clamp(0., 0.999) * 256.;
    let g = (pixel_colour.y() / samples_per_pixel).clamp(0., 0.999) * 256.;
    let b = (pixel_colour.z() / samples_per_pixel).clamp(0., 0.999) * 256.;

    format!("{} {} {}\n", r as u16, g as u16, b as u16) 
}
