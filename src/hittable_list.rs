use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub(crate) struct HittableList {
    objects: Vec<Hittable>,
}

impl HittableList {
    pub(crate) fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, object: Hittable) {
        self.objects.push(object);
    }

    pub(crate) fn clear(&mut self) {
        self.objects.clear();
    }

    pub(crate) fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
