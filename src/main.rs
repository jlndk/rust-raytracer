use glam::Vec3;
use rand::{Rng, rngs::ThreadRng};
use std::time::Instant;
use rayon::prelude::*;
use std::sync::Arc;
use indicatif::{ParallelProgressIterator, ProgressStyle};

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

// Static colors
const WHITE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const BLUE: Vec3 = Vec3::new(0.5, 0.7, 1.0);

// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
// const IMAGE_WIDTH: i32 = 400;
// const IMAGE_WIDTH: i32 = 600;
const IMAGE_WIDTH: i32 = 1920;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

// const SAMPLES_PER_PIXEL: i32 = 5;
// const SAMPLES_PER_PIXEL: i32 = 10;
const SAMPLES_PER_PIXEL: i32 = 50;
// const MAX_DEPTH: i32 = 6;
// const MAX_DEPTH: i32 = 12;
const MAX_DEPTH: i32 = 50;

// Camera
const LOOKFROM: Vec3  = Vec3::new(13.0, 2.0, 3.0);
const LOOKAT: Vec3  = Vec3::new(0.0, 0.0, 0.0);
const VUP: Vec3  = Vec3::new(0.0, 1.0, 0.0);

const FOV: f32  = 20.0;
const APERTURE: f32  = 0.1;
const DIST_TO_FOCUS: f32  = 10.0;

/**
 * COMPUTED CONSTANTS
 */
// The average color of multiple sampels per pixels is computed, so this factor decides what to divide the cumulative sum of all the samples with
const COLOR_SCALE: f32 = 1.0 / SAMPLES_PER_PIXEL as f32;

fn main() {
    // Generate the scene (placing random spheres on a plane).
    // Put it in an ARC to share it across threads
    let shared_world = Arc::new(HittableList::random_scene());

    // Define the Camera
    let camera = Camera::new(LOOKFROM, LOOKAT, VUP, FOV, ASPECT_RATIO, APERTURE, DIST_TO_FOCUS);

    // Start timer to figure out how long the render took
    let start = Instant::now();

    let progress_bar = ProgressStyle::default_bar().template("[{elapsed} ({eta} ETA)] {percent}% {wide_bar} ({pos}/{len} rows)").unwrap();

    // Render all pixels. Render each row in parallel
    let pixels: Vec<Vec<Vec3>> = (0..IMAGE_HEIGHT).into_par_iter().rev().progress_with_style(progress_bar).map(|j| {
        // random number generator
        let mut rng = rand::thread_rng();

        // Grap thread-safe reference to the world
        let world = Arc::clone(&shared_world);

        return (0..IMAGE_WIDTH).map(|i| compute_pixel_color(i, j, &camera, &world, &mut rng)).collect();
    }).collect();

    // Figure out and report how long the render took
    let duration = start.elapsed();
    eprintln!("\nRendering completed in {:?}", duration);

    // Write pixels to stdout
    write_image(pixels);

    eprintln!("Done!");
}

/**
 * Computes the color of a given pixel at coordinate (x, y)
 */
fn compute_pixel_color(y: i32, x: i32, camera: &Camera, world: &HittableList, rng: &mut ThreadRng) -> Vec3 {
    let mut pixel_color = Vec3::ZERO;

    for _s in 0..SAMPLES_PER_PIXEL {
        let u = ((y as f32) + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f32;
        let v = ((x as f32) + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f32;

        let ray = camera.get_ray(u, v);
        pixel_color += compute_ray_color(ray, &world, MAX_DEPTH);
    }

    return Vec3::new(
        (pixel_color.x * COLOR_SCALE).sqrt(),
        (pixel_color.y * COLOR_SCALE).sqrt(),
        (pixel_color.z * COLOR_SCALE).sqrt(),
    );
}

fn compute_ray_color(ray: Ray, world: &HittableList, depth: i32) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let hit_record = world.hit(&ray, 0.001, f32::INFINITY);

    match hit_record {
        Some(rec) => match rec.material.scatter(&ray, &rec) {
            Some(ScatterResult { scattered, attenuation }) => attenuation * compute_ray_color(scattered, &world, depth-1),
            None => Vec3::ZERO,
        },
        None => {
            let t = ray.direction.normalize().y + 1.0;
            return WHITE.lerp(BLUE, t);
        }
    }
}

fn write_image(pixels: Vec<Vec<Vec3>>) {
    println!("P3");
    println!("{0} {1}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{0}", 255);

    for row in pixels {
        for color in row {
            let ir = (255.999 * color.x) as i32;
            let ig = (255.999 * color.y) as i32;
            let ib = (255.999 * color.z) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
