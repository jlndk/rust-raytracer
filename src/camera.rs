use glam::Vec3;

use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32, // vertical field-of-view in degrees
        aspect_ratio: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self { origin, horizontal, vertical, lower_left_corner }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin;

        return Ray::new(self.origin, direction);
    }
}
