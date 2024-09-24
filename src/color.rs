use crate::vec3::Vector3;

pub(crate) fn write_color(pixel_color: Vector3, samples_per_pixel: i32) {
    let r = pixel_color.x / samples_per_pixel as f64;
    let g = pixel_color.y / samples_per_pixel as f64;
    let b = pixel_color.z / samples_per_pixel as f64;

    println!(
        "{} {} {}",
        (256. * r.clamp(0., 0.999)) as i32,
        (256. * g.clamp(0., 0.999)) as i32,
        (256. * b.clamp(0., 0.999)) as i32,
    );
}
