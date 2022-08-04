use glam::Vec3;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3Extension;

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub const fn new(albedo: Vec3) -> Self {
        return Self { albedo };
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = hit_record.normal;

        /*
        If the random unit vector we generate is exactly opposite the normal vector,
        the two will sum to zero, which will result in a zero scatter direction vector.
        This leads to bad scenarios later on (infinities and NaNs), so we only add the
        random component if it is not near zero
        */
        if !scatter_direction.is_near_zero() {
            scatter_direction += Vec3::random_unit_vector();
        }

        let result = ScatterResult {
            scattered: Ray::new(hit_record.point, scatter_direction),
            attenuation: self.albedo,
        };

        return Some(result);
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub const fn new(albedo: Vec3) -> Self {
        return Self { albedo };
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction.normalize().reflect_in(hit_record.normal);

        if reflected.dot(hit_record.normal) <= 0.0 {
            return None;
        }

        let result = ScatterResult {
            scattered: Ray::new(hit_record.point, reflected),
            attenuation: self.albedo,
        };

        return Some(result);
    }
}
