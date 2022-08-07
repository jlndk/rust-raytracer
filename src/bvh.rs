use glam::Vec3;
use rand::Rng;
use std::{cmp::Ordering, mem};

use crate::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
};

pub struct BoundingVolumeHieracy {
    left: Box<dyn Hittable + Send + Sync>,
    right: Box<dyn Hittable + Send + Sync>,
    bounding_box: AxisAlignedBoundingBox,
}

unsafe impl Send for BoundingVolumeHieracy {}
unsafe impl Sync for BoundingVolumeHieracy {}

impl BoundingVolumeHieracy {
    pub fn from_hittable_list(hittable_list: HittableList) -> Self {
        let end = hittable_list.objects.len() - 1;
        return BoundingVolumeHieracy::from_vec(hittable_list.objects, 0, end);
    }

    pub fn from_vec(
        objects: Vec<Box<dyn Hittable + Send + Sync>>,
        start: usize,
        end: usize,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0usize..=2usize);

        let (left, right): (
            Box<dyn Hittable + Send + Sync>,
            Box<dyn Hittable + Send + Sync>,
        ) = match objects.len() {
            length if length == 1 => (objects[0], objects[0]),
            length if length == 2 => {
                let a = objects[0];
                let b = objects[1];
                match box_compare(&a, &b, axis) {
                    Ordering::Less => (a, b),
                    _ => (b, a),
                }
            }
            _ => {
                let mid = objects.len() / 2;

                let left = Box::new(BoundingVolumeHieracy::from_vec(objects, start, mid))
                    as Box<dyn Hittable + Send + Sync>;

                let right = Box::new(BoundingVolumeHieracy::from_vec(objects, mid, end))
                    as Box<dyn Hittable + Send + Sync>;

                (left, right)
            }
        };

        let box_left = left.get_bounding_box();
        let box_right = right.get_bounding_box();

        if box_left.is_none() || box_right.is_none() {
            panic!("Some hittable did not compute a bounding box while constructing BVH tree");
        }

        let bounding_box =
            AxisAlignedBoundingBox::surrounding(box_left.unwrap(), box_right.unwrap());

        return Self {
            left,
            right,
            bounding_box,
        };
    }
}

impl Hittable for BoundingVolumeHieracy {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        return match self.left.hit(ray, t_min, t_max) {
            None => self.right.hit(ray, t_min, t_max),
            hit => hit,
        };
    }

    fn get_bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        // Make a copy since we can't move bounding box outside this object
        return Some(AxisAlignedBoundingBox::new(
            self.bounding_box.minimum,
            self.bounding_box.maximum,
        ));
    }
}

pub struct AxisAlignedBoundingBox {
    minimum: Vec3,
    maximum: Vec3,
}

impl AxisAlignedBoundingBox {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        return Self { minimum, maximum };
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let recip_dir = ray.direction.recip();

        for a in 0..3 {
            let mut t0 = (self.minimum[a] - ray.origin[a]) * recip_dir[a];
            let mut t1 = (self.maximum[a] - ray.origin[a]) * recip_dir[a];

            if recip_dir[a] < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }

            if t1.min(t_max) <= t0.max(t_min) {
                return false;
            }
        }

        return true;
    }

    pub fn surrounding(
        box0: AxisAlignedBoundingBox,
        box1: AxisAlignedBoundingBox,
    ) -> AxisAlignedBoundingBox {
        let small = box0.minimum.min(box1.minimum);
        let big = box0.maximum.max(box1.minimum);

        return AxisAlignedBoundingBox::new(small, big);
    }
}

fn box_compare(
    a: &Box<dyn Hittable + Send + Sync>,
    b: &Box<dyn Hittable + Send + Sync>,
    axis: usize,
) -> Ordering {
    let box_a = a.get_bounding_box();
    let box_b = b.get_bounding_box();

    if box_a.is_none() || box_b.is_none() {
        panic!("Some hittable did not compute a bounding box while constructing BVH tree");
    }

    return box_a.unwrap().minimum[axis].total_cmp(&box_b.unwrap().minimum[axis]);
}
