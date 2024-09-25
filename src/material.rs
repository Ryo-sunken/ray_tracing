use rand_chacha::ChaCha8Rng;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vector3;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Material {
    NONE,
    LAMBERTIAN(Lambertian),
    METAL(Metal),
}

impl Material {
    pub(crate) fn lambertian(albedo: Vector3) -> Self {
        Self::LAMBERTIAN(Lambertian { albedo })
    }

    pub(crate) fn metal(albedo: Vector3, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1. { fuzz } else { 1. };
        Self::METAL(Metal { albedo, fuzz })
    }

    pub(crate) fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
        engine: &mut ChaCha8Rng,
    ) -> bool {
        match self {
            Self::NONE => false,
            Self::LAMBERTIAN(lambertian) => {
                let mut scatter_dir = rec.normal + Vector3::random_unit_vector(engine);

                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal;
                }

                *scattered = Ray::new(rec.p, scatter_dir);
                *attenuation = lambertian.albedo;
                true
            }
            Self::METAL(metal) => {
                let reflected = r.dir.normalized().reflect(rec.normal);
                *scattered = Ray::new(
                    rec.p,
                    reflected + metal.fuzz * Vector3::random_in_unit_sphere(engine),
                );
                *attenuation = metal.albedo;
                scattered.dir.dot(rec.normal) > 0.
            }
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::NONE
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Lambertian {
    albedo: Vector3,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Metal {
    albedo: Vector3,
    fuzz: f64,
}
