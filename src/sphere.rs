use crate::bvh::AxisAlignedBoundingBox;
use std::f32::consts::PI;
use std::f32::consts::TAU;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    pub material: Box<dyn Material>,
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared() as f32;
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() as f32 - self.radius * self.radius;

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

        let (u, v) = Sphere::get_uv(outward_normal);

        let hit_record =
            HitRecord::from_ray(ray, hit_point, outward_normal, root, u, v, &self.material);

        return Some(hit_record);
    }

    fn get_bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
        let min = self.center - radius_vec;
        let max = self.center + radius_vec;
        return Some(AxisAlignedBoundingBox::new(min, max));
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        return Self {
            center,
            radius,
            material,
        };
    }

    fn get_uv(point: Vec3) -> (f32, f32) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        let u = phi / (TAU);
        let v = theta / PI;
        return (u, v);
    }
}
