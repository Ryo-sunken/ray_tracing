mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod vec3;

use camera::Camera;
use color::write_color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use rand::distributions::{Distribution, Standard};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::Uniform;
use ray::Ray;
use vec3::Vector3;

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: i32 = 384;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

fn ray_color(r: &Ray, world: &HittableList) -> Vector3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0., std::f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vector3::one());
    }

    let unit_dir = r.dir.normalized();
    let t = 0.5 * (unit_dir.y + 1.);
    (1. - t) * Vector3::one() + t * Vector3::new(0.5, 0.7, 1.)
}

fn randu(engine: &mut ChaCha8Rng) -> f64 {
    let dist = Uniform::new(0., 1.);
    dist.sample(engine)
}

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let cam = Camera::new();
    let mut world = HittableList::new();
    world.push(Hittable::sphere(Vector3::new(0., 0., -1.), 0.5));
    world.push(Hittable::sphere(Vector3::new(0., -100.5, -1.), 100.));
    let mut engine = ChaCha8Rng::seed_from_u64(123456);

    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            let mut pixel_color = Vector3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (j as f64 + randu(&mut engine)) / (IMAGE_WIDTH - 1) as f64;
                let v = (i as f64 + randu(&mut engine)) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
