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

pub(crate) enum Shape {
    SPHERE(Sphere),
}

pub(crate) struct Hittable {
    pub(crate) shape: Shape,
}

impl Hittable {
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
                        return true;
                    }
                    let temp = (-half_b + root) / a;
                    if t_min < temp && temp < t_max {
                        rec.t = temp;
                        rec.p = r.at(rec.t);
                        let outward_normal = (rec.p - sphere.center) / sphere.radius;
                        rec.set_face_normal(r, outward_normal);
                        return true;
                    }
                }
                return false;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    fn new(center: Vector3, radius: f64) -> Self {
        Self { center, radius }
    }
}
