use crate::ray::Ray;
use crate::vec3::Vector3;

#[derive(Debug, Clone, Copy)]
pub(crate) struct HitRecord {
    pub(crate) p: Vector3,
    pub(crate) normal: Vector3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3) {
        self.front_face = r.dir.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub(crate) trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
