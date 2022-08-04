use glam::Vec3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared() as f32;
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() as f32 - self.radius*self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;

        let hit_record = HitRecord::from_ray(ray, hit_point, outward_normal, root);

        return Some(hit_record);
    }
}

impl Sphere {
    pub fn new( center: Vec3, radius: f32) -> Self {
        return Self { center, radius };
    }
}
