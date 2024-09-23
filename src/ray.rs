use crate::vec3::Vector3;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Ray {
    pub(crate) origin: Vector3,
    pub(crate) dir: Vector3,
}

impl Ray {
    pub(crate) fn new(origin: Vector3, dir: Vector3) -> Self {
        Self { origin, dir }
    }

    pub(crate) fn at(&self, t: f64) -> Vector3 {
        self.origin + self.dir * t
    }
}
