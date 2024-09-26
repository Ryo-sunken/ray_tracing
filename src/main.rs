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

fn randu(engine: &mut ChaCha8Rng) -> f64 {
    let dist = Uniform::new(0., 1.);
    dist.sample(engine)
}

fn random_scene(engine: &mut ChaCha8Rng) -> HittableList {
    let mut world = HittableList::new();

    let ground_mat = Material::lambertian(Vector3::new(0.5, 0.5, 0.5));
    world.push(Hittable::new(
        Shape::sphere(Vector3::new(0., -1000., 0.), 1000.),
        ground_mat,
    ));

    let dist = Uniform::new(0., 1.);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = randu(engine);

            let center = Vector3::new(
                a as f64 + 0.9 * randu(engine),
                0.2,
                b as f64 + 0.9 * randu(engine),
            );

            if (center - Vector3::new(0.4, 0.2, 0.)).length() > 0.9 {
                let sphere_mat = if choose_mat < 0.8 {
                    let albedo = Vector3::randu(&dist, engine) * Vector3::randu(&dist, engine);
                    Material::lambertian(albedo)
                } else if choose_mat < 0.95 {
                    let albedo = Vector3::randu(&dist, engine) * Vector3::randu(&dist, engine);
                    let fuzz = randu(engine) / 2.;
                    Material::metal(albedo, fuzz)
                } else {
                    Material::dielectric(1.5)
                };
                world.push(Hittable::new(Shape::sphere(center, 0.2), sphere_mat));
            }
        }
    }

    let mat1 = Material::dielectric(1.5);
    let mat2 = Material::lambertian(Vector3::new(0.4, 0.2, 0.1));
    let mat3 = Material::metal(Vector3::new(0.7, 0.6, 0.5), 0.);
    world.push(Hittable::new(
        Shape::sphere(Vector3::new(0., 1., 0.), 1.),
        mat1,
    ));
    world.push(Hittable::new(
        Shape::sphere(Vector3::new(-4., 1., 0.), 1.),
        mat2,
    ));
    world.push(Hittable::new(
        Shape::sphere(Vector3::new(4., 1., 0.), 1.),
        mat3,
    ));

    world
}

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

    let cam = Camera::new(
        Vector3::new(13., 2., 3.),
        Vector3::zero(),
        Vector3::unit_y(),
        20.,
        ASPECT_RATIO,
        0.1,
        10.,
    );
    let mut engine = ChaCha8Rng::seed_from_u64(123456);
    let dist = Uniform::new(0., 1.);
    let world = random_scene(&mut engine);

    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            let mut pixel_color = Vector3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (j as f64 + dist.sample(&mut engine)) / (IMAGE_WIDTH - 1) as f64;
                let v = (i as f64 + dist.sample(&mut engine)) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v, &mut engine);
                pixel_color += ray_color(&r, &world, &mut engine, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
