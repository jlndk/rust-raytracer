use glam::Vec3;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};

pub struct RectXY {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Box<dyn Material>,
}

unsafe impl Send for RectXY {}
unsafe impl Sync for RectXY {}

impl RectXY {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Box<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Hittable for RectXY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);

        let point = ray.at(t);
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        return Some(HitRecord::from_ray(
            ray,
            point,
            outward_normal,
            t,
            u,
            v,
            &self.material,
        ));
    }
}
