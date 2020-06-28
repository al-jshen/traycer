use crate::vec3d::Colour;

pub fn get_colour(pixel_colour: Colour) -> String {
    let r = (255.999 * pixel_colour.x()) as u16;
    let g = (255.999 * pixel_colour.y()) as u16;
    let b = (255.999 * pixel_colour.z()) as u16;
    format!("{} {} {}\n", r, g, b) 
}
