use glam::Vec3;
use rand::Rng;

use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::material::Dielectric;
use crate::sphere::Sphere;
use crate::vec3::Vec3Extension;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
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

    pub fn random_scene() -> HittableList {
        let mut world = HittableList::new();

        let mut rng = rand::thread_rng();

        const GROUND_MATERIAL: Lambertian = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
        world.add(
            Box::new(
                Sphere::new(
                    Vec3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    Box::new(GROUND_MATERIAL)
                )
            )
        );

        let center_clear_dist = Vec3::new(4.0, 0.2, 0.0);

        for a in -11..11 {
            for b in -11..11 {
                let x = (a as f32) + 0.9 * rng.gen_range(0.0..=1.0);
                let y = 0.2;
                let z = (b as f32) + 0.9 * rng.gen_range(0.0..=1.0);

                let center = Vec3::new(x, y, z);


                if (center - center_clear_dist).length() > 0.9 {
                    let mut rng = rand::thread_rng();

                    let probability = rng.gen_range(0.0..=1.0);

                    // diffuse
                    if probability < 0.8 {
                        let albedo = Vec3::rand() * Vec3::rand();
                        let material = Lambertian::new(albedo);
                        world.add(Box::new(Sphere::new(center, 0.2, Box::new(material))));
                    }
                    // metal
                    else if probability < 0.95 {
                        let albedo = Vec3::rand_range(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..=0.5);
                        let material = Metal::new(albedo, fuzz);
                        world.add(Box::new(Sphere::new(center, 0.2, Box::new(material))));
                    }
                    // glass
                    else {
                        let material = Dielectric::new(1.5);
                        world.add(Box::new(Sphere::new(center, 0.2, Box::new(material))));
                    }

                }

            }
        }

        const MATERIAL1: Dielectric = Dielectric::new(1.5);
        world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(MATERIAL1))));

        const MATERIAL2: Lambertian = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
        world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(MATERIAL2))));

        const MATERIAL3: Metal = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
        world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(MATERIAL3))));

        return world;
    }
}
