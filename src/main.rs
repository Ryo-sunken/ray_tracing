mod color;
mod ray;
mod vec3;

use crate::color::write_color;
use crate::ray::Ray;
use crate::vec3::Vector3;

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: i32 = 384;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn ray_color(r: &Ray) -> Vector3 {
    if hit_sphere(&Vector3::new(0., 0., -1.), 0.5, r) {
        return Vector3::new(1., 0., 0.);
    }
    let unit_dir = r.dir.normalized();
    let t = 0.5 * (unit_dir.y + 1.);
    (1. - t) * Vector3::new(0.5, 0.7, 1.) + t * Vector3::new(1., 1., 1.)
}

fn hit_sphere(center: &Vector3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - *center;
    let a = r.dir.length_squared();
    let b = 2. * oc.dot(&r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant > 0.
}

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let viewport_height = 2.;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.;

    let origin = Vector3::zero();
    let horizontal = Vector3::new(viewport_width, 0., 0.);
    let vertical = Vector3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vector3::new(0., 0., focal_length);

    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let u = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);

            write_color(&pixel_color);
        }
    }
}
