mod color;
mod vec3;

use crate::color::write_color;
use crate::vec3::Vector3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let pixel_color = Vector3::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );

            write_color(&pixel_color);
        }
    }
}
