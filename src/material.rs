use rand_chacha::ChaCha8Rng;
use rand_distr::Distribution;
use rand_distr::Uniform;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vector3;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Material {
    NONE,
    LAMBERTIAN(Lambertian),
    METAL(Metal),
    DIELECTRIC(Dielectric),
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Material {
    pub(crate) fn lambertian(albedo: Vector3) -> Self {
        Self::LAMBERTIAN(Lambertian { albedo })
    }

    pub(crate) fn metal(albedo: Vector3, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1. { fuzz } else { 1. };
        Self::METAL(Metal { albedo, fuzz })
    }

    pub(crate) fn dielectric(ref_idx: f64) -> Self {
        Self::DIELECTRIC(Dielectric { ref_idx })
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
            Self::DIELECTRIC(dielectric) => {
                *attenuation = Vector3::one();
                let ratio = if rec.front_face {
                    dielectric.ref_idx.recip()
                } else {
                    dielectric.ref_idx
                };
                let unit_dir = r.dir.normalized();
                let cos_theta = -unit_dir.dot(rec.normal).min(1.);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();
                let reflect_prob = schlick(cos_theta, ratio);
                if ratio * sin_theta > 1. {
                    let reflected = unit_dir.reflect(rec.normal);
                    *scattered = Ray::new(rec.p, reflected);
                } else if Uniform::new(0., 1.).sample(engine) < reflect_prob {
                    let reflected = unit_dir.reflect(rec.normal);
                    *scattered = Ray::new(rec.p, reflected);
                } else {
                    let refracted = unit_dir.refract(rec.normal, ratio);
                    *scattered = Ray::new(rec.p, refracted);
                }
                true
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct Dielectric {
    ref_idx: f64,
}
