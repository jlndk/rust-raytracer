use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;

        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        return temp_rec;
    }
}

impl HittableList {
    pub fn new() -> Self {
        return Self {
            objects: Vec::new(),
        };
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}
