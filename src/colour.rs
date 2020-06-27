use crate::vec3d::Colour;

pub fn write_colour(pixel_colour: Colour) {
    print!("{} ", (255.999 * pixel_colour.x()) as u16);
    print!("{} ", (255.999 * pixel_colour.y()) as u16);
    print!("{}\n", (255.999 * pixel_colour.z()) as u16);

}
