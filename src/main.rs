use glam::Vec3;
use rand::Rng;

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
use sphere::Sphere;
use camera::Camera;
use material::ScatterResult;
use material::Lambertian;
use material::Metal;


// STATIC COLORS
const WHITE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const BLUE: Vec3 = Vec3::new(0.5, 0.7, 1.0);


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    // let image_width = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let samples_per_pixel = 50;
    let max_depth = 12;

    // World
    let mut world = HittableList::new();

    const MATERIAL_GROUND: Lambertian = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    const MATERIAL_CENTER: Lambertian = Lambertian::new(Vec3::new(0.7, 0.3, 0.3));
    const MATERIAL_LEFT: Metal = Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3);
    const MATERIAL_RIGHT: Metal = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3);

    world.add(Box::new(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, &MATERIAL_GROUND)));
    world.add(Box::new(Sphere::new(Vec3::new( 0.0,    0.0, -1.0),   0.5, &MATERIAL_CENTER)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.5, &MATERIAL_LEFT)));
    world.add(Box::new(Sphere::new(Vec3::new( 1.0,    0.0, -1.0),   0.5, &MATERIAL_RIGHT)));

    // Camera
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let camera = Camera::new(aspect_ratio, viewport_height, focal_length);

    println!("P3");
    println!("{0} {1}", image_width, image_height);
    println!("{0}", 255);

    let color_scale = 1.0 / samples_per_pixel as f32;

    let mut rng = rand::thread_rng();

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

    eprintln!("\nDone!");
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
