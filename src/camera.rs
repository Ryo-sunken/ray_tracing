use rand_chacha::ChaCha8Rng;

use crate::ray::Ray;
use crate::vec3::Vector3;

pub(crate) struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub(crate) fn new(
        origin: Vector3,
        lookat: Vector3,
        vup: Vector3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let h = (vfov.to_radians() / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - lookat).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_dist * w;
        let lens_radius = aperture / 2.;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub(crate) fn get_ray(&self, s: f64, t: f64, engine: &mut ChaCha8Rng) -> Ray {
        let rd = self.lens_radius * Vector3::random_in_unit_disc(engine);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
