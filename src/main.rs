use glam::Vec3;
use rand::Rng;
use std::time::Instant;

mod ray;
mod hittable_list;
mod hittable;
mod sphere;
mod camera;
mod vec3;
mod material;

use ray::Ray;
use hittable_list::HittableList;
use hittable::Hittable;
use camera::Camera;
use material::ScatterResult;

// STATIC COLORS
const WHITE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const BLUE: Vec3 = Vec3::new(0.5, 0.7, 1.0);


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    // let image_width = 400;
    let image_width = 600;
    // let image_width = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // let samples_per_pixel = 50;
    let samples_per_pixel = 10;
    let max_depth = 12;

    // World
    let world = HittableList::random_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let fov = 20.0;
    let aperture = 0.1;
    let dist_to_focus = 10.0;

    let camera = Camera::new(lookfrom, lookat, vup, fov, aspect_ratio, aperture, dist_to_focus);

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("{0}", 255);

    let color_scale = 1.0 / samples_per_pixel as f32;

    let mut rng = rand::thread_rng();

    let start = Instant::now();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {0}     ", j);

        for i in 0..image_width {
            let mut pixel_color = Vec3::ZERO;

            // TODO: Optimize this loop for SIMD
            for _s in 0..samples_per_pixel {
                let u = ((i as f32) + rng.gen_range(0.0..1.0)) / (image_width - 1) as f32;
                let v = ((j as f32) + rng.gen_range(0.0..1.0)) / (image_height - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, &world, max_depth);
            }

            let scaled_color = Vec3::new(
                (pixel_color.x * color_scale).sqrt(),
                (pixel_color.y * color_scale).sqrt(),
                (pixel_color.z * color_scale).sqrt(),
            );
            write_color(scaled_color);
        }
    }

    let duration = start.elapsed();

    eprintln!("\nRendering completed in {:?}", duration);

    eprintln!("Done!");
}

fn ray_color(ray: Ray, world: &HittableList, depth: i32) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let hit_record = world.hit(&ray, 0.001, f32::INFINITY);

    match hit_record {
        Some(rec) => match rec.material.scatter(&ray, &rec) {
            Some(ScatterResult {scattered, attenuation}) => attenuation * ray_color(scattered, &world, depth-1),
            None => Vec3::ZERO,
        },
        None => {
            let t = ray.direction.normalize().y + 1.0;
            return WHITE.lerp(BLUE, t);
        }
    }
}

fn write_color(color: Vec3) {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    println!("{} {} {}", ir, ig, ib);
}
