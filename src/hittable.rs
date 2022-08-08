use glam::Vec3;

use crate::bvh::AxisAlignedBoundingBox;
use crate::material::Material;
use crate::ray::Ray;

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub material: &'a Box<dyn Material>,
}

impl<'a> HitRecord<'a> {
    pub fn from_ray(
        ray: &Ray,
        point: Vec3,
        outward_normal: Vec3,
        t: f32,
        u: f32,
        v: f32,
        material: &'a Box<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        return Self {
            point,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn get_bounding_box(&self) -> Option<AxisAlignedBoundingBox>;
}
