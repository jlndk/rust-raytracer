use glam::Vec3;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use term_table::{
    row::Row, table_cell::Alignment, table_cell::TableCell, TableBuilder, TableStyle,
};

mod bvh;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod texture;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use material::ScatterResult;
use ray::Ray;

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
// const SAMPLES_PER_PIXEL: i32 = 50;
const SAMPLES_PER_PIXEL: i32 = 100;

// const MAX_DEPTH: i32 = 3;
// const MAX_DEPTH: i32 = 6;
// const MAX_DEPTH: i32 = 12;
const MAX_DEPTH: i32 = 50;

// Camera
const LOOKFROM: Vec3 = Vec3::new(13.0, 2.0, 3.0);
const LOOKAT: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

const FOV: f32 = 20.0;
const APERTURE: f32 = 0.1;
const DIST_TO_FOCUS: f32 = 10.0;

/**
 * COMPUTED CONSTANTS
 */
// The average color of multiple sampels per pixels is computed, so this factor decides what to divide the cumulative sum of all the samples with
const COLOR_SCALE: f32 = 1.0 / SAMPLES_PER_PIXEL as f32;

fn main() {
    // First of all, print the relavant rendering constants to the user
    print_rendering_info();

    // Generate the scene (placing random spheres on a plane).
    // Put it in an ARC to share it across threads
    let shared_world = Arc::new(HittableList::random_scene());

    // Define the Camera
    let camera = Camera::new(
        LOOKFROM,
        LOOKAT,
        VUP,
        FOV,
        ASPECT_RATIO,
        APERTURE,
        DIST_TO_FOCUS,
    );

    // Start timer to figure out how long the render took
    let start = Instant::now();

    // Define styling for the rendering progress bar
    let progress_bar = ProgressStyle::default_bar()
        .template("{percent}% ({pos}/{len} rows) {wide_bar} [{elapsed} ({eta} ETA)]")
        .unwrap();

    // Render all pixels. Render each row in parallel
    let pixels: Vec<Vec<Vec3>> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .progress_with_style(progress_bar)
        .map(|j| {
            // random number generator
            let mut rng = rand::thread_rng();

            // Grap thread-safe reference to the world
            let world = Arc::clone(&shared_world);

            return (0..IMAGE_WIDTH)
                .map(|i| compute_pixel_color(i, j, &camera, &world, &mut rng))
                .collect();
        })
        .collect();

    // Figure out and report how long the render took
    let duration = start.elapsed();
    eprintln!("Rendering completed in {:?}", duration);

    // Write pixels to stdout
    write_image(pixels);

    eprintln!("Done!");
}

fn print_rendering_info() {
    let table = TableBuilder::new()
        .style(TableStyle::extended())
        .rows(vec![
            Row::new(vec![TableCell::new_with_alignment(
                "Rendering information",
                2,
                Alignment::Center,
            )]),
            Row::new(vec![
                TableCell::new("Image resolution"),
                TableCell::new(format!("{}x{}", IMAGE_WIDTH, IMAGE_HEIGHT)),
            ]),
            Row::new(vec![
                TableCell::new("Number of samples per pixel"),
                TableCell::new(format!("{}", SAMPLES_PER_PIXEL)),
            ]),
            Row::new(vec![
                TableCell::new("Maximum amount of light bounces per ray"),
                TableCell::new(format!("{}", MAX_DEPTH)),
            ]),
        ])
        .build();

    eprintln!("{}", table.render());
}

/**
 * Computes the color of a given pixel at coordinate (x, y)
 */
fn compute_pixel_color(
    y: i32,
    x: i32,
    camera: &Camera,
    world: &HittableList,
    rng: &mut ThreadRng,
) -> Vec3 {
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
            Some(ScatterResult {
                scattered,
                attenuation,
            }) => attenuation * compute_ray_color(scattered, &world, depth - 1),
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
