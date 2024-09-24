mod color;
mod hittable;
mod hittable_list;
mod ray;
mod vec3;

use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;

use crate::color::write_color;
use crate::ray::Ray;
use crate::vec3::Vector3;

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: i32 = 384;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn ray_color(r: &Ray, world: &HittableList) -> Vector3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0., std::f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vector3::one());
    }

    let unit_dir = r.dir.normalized();
    let t = 0.5 * (unit_dir.y + 1.);
    (1. - t) * Vector3::one() + t * Vector3::new(0.5, 0.7, 1.)
}

fn hit_sphere(center: Vector3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.dir.length_squared();
    let half_b = oc.dot(r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
        -1.
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let viewport_height = 2.;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.;

    let origin = Vector3::zero();
    let horizontal = viewport_width * Vector3::unit_x();
    let vertical = viewport_height * Vector3::unit_y();
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - focal_length * Vector3::unit_z();

    let mut world = HittableList::new();
    world.push(Hittable::sphere(Vector3::new(0., 0., -1.), 0.5));
    world.push(Hittable::sphere(Vector3::new(0., -100.5, -1.), 100.));

    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            let u = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            write_color(&ray_color(&r, &world));
        }
    }
}
