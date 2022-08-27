use crate::bvh::AxisAlignedBoundingBox;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
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

    fn get_bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        if self.objects.is_empty() {
            return None;
        }

        return (&self.objects).into_iter().fold(None, |b, object| {
            match object.get_bounding_box() {
                Some(object_box) => match b {
                    None => Some(object_box),
                    Some(existing_box) => Some(AxisAlignedBoundingBox::surrounding(
                        existing_box,
                        object_box,
                    )),
                },
                None => None,
            }
        });
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
