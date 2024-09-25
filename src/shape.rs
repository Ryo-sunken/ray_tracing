use crate::vec3::Vector3;

pub(crate) enum Shape {
    SPHERE(Sphere),
}

impl Shape {
    pub(crate) fn sphere(center: Vector3, radius: f64) -> Self {
        Self::SPHERE(Sphere { center, radius })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Sphere {
    pub(crate) center: Vector3,
    pub(crate) radius: f64,
}
