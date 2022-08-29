use glam::Vec3;
use rand::Rng;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::Vec3Extension;

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;

    fn emitted(&self, _u: f32, _v: f32, _point: Vec3) -> Vec3 {
        return Vec3::ZERO; // Black
    }
}

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub const fn new(albedo: Box<dyn Texture>) -> Self {
        return Self { albedo };
    }

    pub fn from_color(color: Vec3) -> Self {
        return Self::new(Box::new(SolidColor::new(color)));
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
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
            attenuation: self
                .albedo
                .value(hit_record.u, hit_record.v, &hit_record.point),
        };

        return Some(result);
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub const fn new(albedo: Vec3, fuzz: f32) -> Self {
        return Self { albedo, fuzz };
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction.normalize().reflect_in(hit_record.normal);

        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if scattered.direction.dot(hit_record.normal) <= 0.0 {
            return None;
        }

        let result = ScatterResult {
            scattered,
            attenuation: self.albedo,
        };

        return Some(result);
    }
}

pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub const fn new(index_of_refraction: f32) -> Self {
        return Self {
            index_of_refraction,
        };
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let refraction_ratio = match hit_record.front_face {
            true => (1.0 / self.index_of_refraction),
            false => self.index_of_refraction,
        };

        let can_refract = refraction_ratio * sin_theta <= 1.0;

        let mut rng = rand::thread_rng();

        let direction = match can_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            true => unit_direction.refract_off(hit_record.normal, refraction_ratio),
            false => unit_direction.reflect_in(hit_record.normal),
        };

        let result = ScatterResult {
            scattered: Ray::new(hit_record.point, direction),
            attenuation: Vec3::ONE,
        };

        return Some(result);
    }
}

pub struct DiffuseLight {
    texture: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        return Self { texture: texture };
    }

    pub fn from_color(color: Vec3) -> Self {
        return DiffuseLight::new(Box::new(SolidColor::new(color)));
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        return None;
    }

    fn emitted(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        return self.texture.value(u, v, &point);
    }
}
