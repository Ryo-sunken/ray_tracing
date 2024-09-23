use crate::vec3::Vector3;

pub(crate) fn write_color(pixel_color: &Vector3) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x) as i32,
        (255.999 * pixel_color.y) as i32,
        (255.999 * pixel_color.z) as i32
    );
}
