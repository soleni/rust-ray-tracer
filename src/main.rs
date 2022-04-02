#![allow(unused_imports)]

extern crate image;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

use indicatif::ProgressBar;

use std::io;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use threadpool::ThreadPool;
use std::sync::Mutex;
use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};

mod vec3;
mod ray;
mod sphere;
mod hitable;
mod hitablelist;
mod camera;
mod materials;

use rand::prelude::*;
use vec3::*;
use ray::*;
use sphere::*;
use hitable::*;
use hitablelist::*;
use camera::*;
use materials::*;

static NX: u32 = 800;
static NY: u32 = 400;
static NS: u32 = 30;

static THREADS_NUM: usize = 6;

// get color for ray
fn color(ray : &Ray, world: &HitableList, depth: i32) -> Vec3 {
    let mut rec = HitRecord{t: 0.0, p: unit_vector(), normal: unit_vector(), material: &Metal{albedo: unit_vector()}};
    if world.hit(ray, 0.001, f32::MAX, &mut rec) {
        let mut scattered: Ray = Ray{a: unit_vector(), b: unit_vector()};
        let mut attenuation: Vec3 = unit_vector();
        if depth < 50 && rec.material.scatter(ray, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return unit_vector();
        }
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return make_vec3(1.0, 1.0,1.0) * (1.0 - t) + make_vec3(0.2, 0.5, 1.0) * t;
    }
}

// launch precess ray to pixel (x,y)
fn make_ray(camera: &Camera, world: &HitableList, x: u32, y: u32) -> Vec3 {
    let mut col = make_vec3(0.0, 0.0, 0.0);
    for _s in 0..NS {
        let mut rng = rand::thread_rng();
        let u = (x as f32 + rng.gen::<f32>() as f32) / NX as f32;
        let v = (y as f32 + rng.gen::<f32>() as f32) / NY as f32;
        let r = camera.get_ray(u, v);
        let _p = r.point_at_parameter(2.0);
        col += color(&r, world, 0);
    }
    col /= NS as f32;
    col = col.sqrt();
    col *= 255.99;

    col
}

fn main() -> io::Result<()> {
    let now = Instant::now();

    // Png output image buffer
    let mut img_buffer = ImageBuffer::new(NX, NY);

    // Prerender objects
    let m1 = Metal{albedo: make_vec3(0.8, 0.6, 0.2)}; 
    let m2 = Lambertian{albedo: make_vec3(0.8, 0.8, 0.0)};

    let s1 =  Sphere{center: make_vec3(0.0, 0.0, -1.0), radius: 0.5, material: &m1};
    let s2 =  Sphere{center: make_vec3(-4.0, 0.0, -1.0), radius: 0.5, material: &m1};
    let s3 =  Sphere{center: make_vec3(3.0, 3.0, -5.0), radius: 2.0, material: &m1};
    let s4 = Sphere{center: make_vec3(0.0, -100.5, -1.0), radius: 100.0, material: &m2};

    let hitable: Vec<&dyn Hitable> = vec![&s1, &s2, &s3, &s4];

    let world = make_hitable_list(hitable, 2);
    let camera = standart_camera();
 
    // Progress Bar
    let bar = ProgressBar::new(100);
    let bar_counter = ((NX * NY) as f32 / 100.0) as u32;

    for j in (0..NY).rev() {
        for i in 0..NX {
            let col = make_ray(&camera, &world, i, j);
            img_buffer.put_pixel(NX - i - 1, NY - j - 1, image::Rgb([col.x as u8, col.y as u8, col.z as u8]));           

            if (j*NX + i) % bar_counter == 0 {
                bar.inc(1);
            }
        }
    } 
    bar.finish();

    img_buffer.save("output.png").unwrap();

    println!("Time elapsed: {} sec.", now.elapsed().as_secs());
    Ok(())
}