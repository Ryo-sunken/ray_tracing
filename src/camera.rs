use crate::ray::Ray;
use crate::vec3::Vector3;

pub(crate) struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub(crate) fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let h = (vfov.to_radians() / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;
        let origin = Vector3::zero();
        let horizontal = viewport_width * Vector3::unit_x();
        let vertical = viewport_height * Vector3::unit_y();
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - focal_length * Vector3::unit_z();

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub(crate) fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
