use std::f32::consts::TAU;

use glam::Vec3;
use rand::Rng;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::Dielectric;
use crate::material::DiffuseLight;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::sphere::Sphere;
use crate::texture::CheckerTexture;
use crate::texture::SolidColor;
use crate::vec3::Vec3Extension;
use crate::ASPECT_RATIO;

pub struct Scene {
    pub world: HittableList,
    pub camera: Camera,
    pub background: Vec3,
}

pub fn random_scene() -> Scene {
    let mut world = HittableList::new();

    let mut rng = rand::thread_rng();

    let checker_texture = CheckerTexture::new(
        Box::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
    );
    let ground_material = Lambertian::new(Box::new(checker_texture));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(ground_material),
    )));

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
                    let material = Lambertian::new(Box::new(SolidColor::new(albedo)));
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

    let material_1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(material_1),
    )));

    let material_2 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1))));

    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(material_2),
    )));

    let material_3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(material_3),
    )));

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let fov = 20.0;
    let aperture = 0.1;
    let dist_to_focus = 10.0;

    // Define the Camera
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    return Scene {
        world,
        camera,
        background: Vec3::new(0.7, 0.8, 1.0),
    };
}

pub fn random_spheres() -> Scene {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        10.0,
        Box::new(Lambertian::new(Box::new(CheckerTexture::new(
            Box::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
            Box::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
        )))),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 11.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(1.0, 1.0, 1.0), 0.35)),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(1.25, 10.5, 1.0),
        0.4,
        Box::new(Lambertian::new(Box::new(SolidColor::new(Vec3::new(
            0.7, 0.2, 0.3,
        ))))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.5, 10.3, -1.0125),
        0.3,
        Box::new(Lambertian::new(Box::new(SolidColor::new(Vec3::new(
            0.2, 0.3, 0.7,
        ))))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.4, 10.3, -0.5),
        0.2,
        Box::new(Dielectric::new(1.5)),
    )));

    let center_clear_dist = Vec3::new(4.0, 0.2, 0.0);

    let mut rng = rand::thread_rng();

    for _i in 0..1500 {
        let u = rng.gen_range(0.0..=1.0f32);
        let v = rng.gen_range(0.0..=1.0f32);
        let theta = (TAU * u).abs();
        let phi = (2.0 * v - 1.0).acos();

        let x = 10.1 * phi.sin() * theta.cos();
        let y = 10.1 * phi.sin() * theta.sin();
        let z = 10.1 * phi.cos();

        let center = Vec3::new(x, y, z);

        if (center - center_clear_dist).length() > 0.9 {
            let mut rng = rand::thread_rng();

            let probability = rng.gen_range(0.0..=1.0);

            // diffuse
            if probability < 0.8 {
                let albedo = Vec3::rand() * Vec3::rand();
                let material = Lambertian::new(Box::new(SolidColor::new(albedo)));
                world.add(Box::new(Sphere::new(center, 0.1, Box::new(material))));
            }
            // metal
            else if probability < 0.95 {
                let albedo = Vec3::rand_range(0.5, 1.0);
                let fuzz = rng.gen_range(0.0..=0.5);
                let material = Metal::new(albedo, fuzz);
                world.add(Box::new(Sphere::new(center, 0.1, Box::new(material))));
            }
            // glass
            else {
                let material = Dielectric::new(1.5);
                world.add(Box::new(Sphere::new(center, 0.1, Box::new(material))));
            }
        }
    }

    // Camera
    let lookfrom = Vec3::new(10.0, 14.0, 2.0);
    // let lookfrom = Vec3::new(10.0, 20.0, 30.0);
    let lookat = Vec3::new(0.0, 10.5, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let fov = 20.0;
    let aperture = 0.25;
    let dist_to_focus = (lookfrom - lookat).length();

    // Define the Camera
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    return Scene {
        world,
        camera,
        background: Vec3::new(0.7, 0.8, 1.0),
    };
}

pub fn glowing_sphere() -> Scene {
    let mut world = HittableList::new();

    let checker_texture = CheckerTexture::new(
        Box::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
    );
    let ground_material = Lambertian::new(Box::new(checker_texture));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(ground_material),
    )));

    let material_1 = DiffuseLight::from_color(Vec3::new(1.0, 1.0, 1.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(material_1),
    )));

    let material_2 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1))));

    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(material_2),
    )));

    let material_3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.5, 0.5, 1.5),
        0.5,
        Box::new(material_3),
    )));

    let material_4 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Vec3::new(1.5, 0.75, -1.5),
        0.75,
        Box::new(material_4),
    )));

    // Camera
    let lookfrom = Vec3::new(6.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let fov = 30.0;
    let aperture = 0.1;
    let dist_to_focus = (lookfrom - lookat).length();

    // Define the Camera
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    return Scene {
        world,
        camera,
        // background: Vec3::new(0.7, 0.8, 1.0),
        background: Vec3::ZERO,
    };
}
