mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod shape;
mod vec3;

use camera::Camera;
use color::write_color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::Material;
use rand::distributions::Distribution;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::Uniform;
use ray::Ray;
use shape::Shape;
use vec3::Vector3;

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(r: &Ray, world: &HittableList, engine: &mut ChaCha8Rng, depth: i32) -> Vector3 {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Vector3::zero();
    }

    if world.hit(r, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vector3::zero(), Vector3::zero());
        let mut attenuation = Vector3::zero();
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered, engine)
        {
            return attenuation * ray_color(&scattered, world, engine, depth - 1);
        }
        return Vector3::zero();
    }

    let unit_dir = r.dir.normalized();
    let t = 0.5 * (unit_dir.y + 1.);
    (1. - t) * Vector3::one() + t * Vector3::new(0.5, 0.7, 1.)
}

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let cam = Camera::new(90., ASPECT_RATIO);
    let mut world = HittableList::new();
    let r = (std::f64::consts::PI / 4.).cos();
    world.push(Hittable::new(Shape::sphere(Vector3::new(-r, 0., -1.), r), Material::lambertian(Vector3::unit_z())));
    world.push(Hittable::new(Shape::sphere(Vector3::new(r, 0., -1.), r), Material::lambertian(Vector3::unit_x())));
    let mut engine = ChaCha8Rng::seed_from_u64(123456);
    let dist = Uniform::new(0., 1.);

    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            let mut pixel_color = Vector3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (j as f64 + dist.sample(&mut engine)) / (IMAGE_WIDTH - 1) as f64;
                let v = (i as f64 + dist.sample(&mut engine)) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, &mut engine, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
