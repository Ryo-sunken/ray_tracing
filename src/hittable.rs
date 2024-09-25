use crate::material::Material;
use crate::ray::Ray;
use crate::shape::*;
use crate::vec3::Vector3;

#[derive(Debug, Clone, Copy)]
pub(crate) struct HitRecord {
    pub(crate) p: Vector3,
    pub(crate) normal: Vector3,
    pub(crate) material: Material,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vector3::zero(),
            normal: Vector3::zero(),
            material: Material::default(),
            t: 0.,
            front_face: false,
        }
    }
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

pub(crate) struct Hittable {
    pub(crate) shape: Shape,
    pub(crate) material: Material,
}

impl Hittable {
    pub(crate) fn new(shape: Shape, material: Material) -> Self {
        Self { shape, material }
    }

    pub(crate) fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self.shape {
            Shape::SPHERE(sphere) => {
                let oc = r.origin - sphere.center;
                let a = r.dir.length_squared();
                let half_b = oc.dot(r.dir);
                let c = oc.length_squared() - sphere.radius * sphere.radius;
                let discriminant = half_b * half_b - a * c;

                if discriminant > 0. {
                    let root = discriminant.sqrt();
                    let temp = (-half_b - root) / a;
                    if t_min < temp && temp < t_max {
                        rec.t = temp;
                        rec.p = r.at(rec.t);
                        let outward_normal = (rec.p - sphere.center) / sphere.radius;
                        rec.set_face_normal(r, outward_normal);
                        rec.material = self.material;
                        return true;
                    }
                    let temp = (-half_b + root) / a;
                    if t_min < temp && temp < t_max {
                        rec.t = temp;
                        rec.p = r.at(rec.t);
                        let outward_normal = (rec.p - sphere.center) / sphere.radius;
                        rec.set_face_normal(r, outward_normal);
                        rec.material = self.material;
                        return true;
                    }
                }
                return false;
            }
        }
    }
}
