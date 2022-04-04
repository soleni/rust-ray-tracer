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

static NX: u32 = 1200;
static NY: u32 = 800;
static NS: u32 = 1000;

static THREADS_NUM: usize = 6;

// get color for ray
fn color(ray : &Ray, world: &HitableList, depth: i32) -> Vec3 {
    let mut rec = HitRecord{t: 0.0, p: unit_vector(), normal: unit_vector(), material: &Metal{albedo: unit_vector()}};
    if world.hit(ray, 0.001, f32::MAX, &mut rec) {
        let mut scattered: Ray = Ray{a: unit_vector(), b: unit_vector()};
        let mut attenuation: Vec3 = unit_vector();
        if depth < 20 && rec.material.scatter(ray, &mut rec, &mut attenuation, &mut scattered) {
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

fn create_scene() -> Vec<Arc<dyn Hitable>> {
    let mut rng = rand::thread_rng();

    let mut list: Vec<Arc<dyn Hitable>> = vec![];
    
    list.push(Arc::new(Sphere::new(&Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(&Vec3::new(0.1, 0.2, 0.5))))));

    for a in -11..11 {
        for b in -11..11 {
            let chose_mat = rng.gen::<f32>();
            let center = Vec3::new(a as f32 + 0.9*rng.gen::<f32>(), 0.2, b as f32 + 0.9*rng.gen::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if chose_mat < 0.75 {
                    let mut prop = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
                    prop *= prop;
                    list.push(Arc::new(Sphere::new(&center, 0.2, Arc::new(Lambertian::new(&prop)))));
                } else if chose_mat < 0.9 {
                    let mut prop = Vec3::new(rng.gen::<f32>() + 1.0, rng.gen::<f32>() + 1.0, rng.gen::<f32>() + 1.0);
                    prop *= 0.5;
                    list.push(Arc::new(Sphere::new(&center, 0.2, Arc::new(Metal::new(&prop)))));
                } else {
                    list.push(Arc::new(Sphere::new(&center, 0.2, Arc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    list.push(Arc::new(Sphere::new(&Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric::new(1.5)))));
    list.push(Arc::new(Sphere::new(&Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(&Vec3::new(0.4, 0.2, 0.1))))));
    list.push(Arc::new(Sphere::new(&Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(&Vec3::new(0.7, 0.6, 0.5))))));
    
    list 
}

fn main() -> io::Result<()> {
    let now = Instant::now();

    // Png output image buffer
    let img_buffer = Arc::new(Mutex::new(ImageBuffer::new(NX, NY)));

    // Objects
    let hitable = create_scene();
    let world = HitableList{list: hitable};

    let lookfrom = Vec3::new(8.0, 2.0, -2.0);
    let lookat = Vec3::new(0.0, 0.5, 0.0);
    let dist_to_focus = 8.0;
    let aperture = 0.0; // no focus
    let camera = Arc::new(Camera::new(
        lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0),
        50.0, NX as f32 / NY as f32, 
        aperture, dist_to_focus)
    );
 
    // Progress Bar
    let bar = ProgressBar::new(100);
    let bar_counter = ((NX * NY) as f32 / 100.0) as u32;

    let pool = ThreadPool::new(THREADS_NUM);
    for j in (0..NY).rev() {
        for i in 0..NX {
            let world = world.clone();
            let camera = camera.clone();
            let img_buffer = img_buffer.clone();

            pool.execute(move|| {
                let col = make_ray(&camera, &world, i, j);
                img_buffer.lock().unwrap().put_pixel(NX - i - 1, NY - j - 1, image::Rgb([col.x as u8, col.y as u8, col.z as u8]));                        
            });

            if (j*NX + i) % bar_counter == 0 {
                bar.inc(1);
            }
        }
        pool.join();
    } 
    bar.finish();

    img_buffer.lock().unwrap().save("output.png").unwrap();

    println!("Time elapsed: {} sec.", now.elapsed().as_secs());
    Ok(())
}