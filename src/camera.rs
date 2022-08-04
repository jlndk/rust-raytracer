use glam::Vec3;

use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, viewport_height: f32, focal_length: f32) -> Self {
        let viewport_width = viewport_height * aspect_ratio;
        let origin = Vec3::ZERO;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self { origin, horizontal, vertical, lower_left_corner }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;

        return Ray::new(self.origin, direction);
    }
}
