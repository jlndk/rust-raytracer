use glam::Vec3;
use rand::Rng;

mod ray;
mod hittable_list;
mod hittable;
mod sphere;
mod camera;
mod vec3;

use ray::Ray;
use hittable_list::HittableList;
use hittable::Hittable;
use sphere::Sphere;
use camera::Camera;
use vec3::RandomVec3;


// STATIC COLORS
const WHITE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const BLUE: Vec3 = Vec3::new(0.5, 0.7, 1.0);


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let samples_per_pixel = 20;
    let max_depth = 12;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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
        Some(rec) => {
            let target = rec.point + rec.normal + Vec3::random_unit_vector();

            let ray = Ray::new(rec.point, target - rec.point);

            return 0.5 * ray_color(ray, &world, depth - 1);
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
