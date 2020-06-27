fn main() {
    let image_width: u16 = 256;
    let image_height: u16 = 256;
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = (i as f32) / (image_width - 1) as f32;
            let g = (j as f32) / (image_height - 1) as f32;
            let b = 0.25 as f32;

            let ir = (255.999 * r).floor() as u16;
            let ig = (255.999 * g).floor() as u16;
            let ib = (255.999 * b).floor() as u16;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
