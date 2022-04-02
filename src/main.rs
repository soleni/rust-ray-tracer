#![allow(unused_imports)]

use std::io;
use std::fs::File;
use std::io::Write;

mod vec3;
mod ray;
mod sphere;
mod hitable;
mod hitablelist;
mod camera;

use rand::prelude::*;
use vec3::*;
use ray::*;
use sphere::*;
use hitable::*;
use hitablelist::*;
use camera::*;

use indicatif::ProgressBar;



fn random_in_unit_sphere() -> Vec3{
    let mut p: Vec3;
    let mut rng = rand::thread_rng();
    loop {
        p = make_vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        if p.squared_length() > 1.0 {
            break;
        }
    }
    p
}

fn color<T: Hitable>(ray : &Ray, world: &HitableList<T>) -> Vec3 {
    let mut rec = HitRecord{t: 0.0, p: unit_vector(), normal: unit_vector()};
    if world.hit(ray, 0.0, f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return color( &Ray{a: rec.p, b: target - rec.p}, &world) * 0.5;
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return make_vec3(1.0, 1.0,1.0) * (1.0 - t) + make_vec3(0.2, 0.5, 1.0) * t;
    }
}

fn main() -> io::Result<()> {
    let mut writer = File::create("output.ppm").unwrap();
    let mut rng = rand::thread_rng();

    let nx = 400;
    let ny = 200;
    let ns = 50;

    write!(&mut writer, "P3\n{} {}\n255\n",nx, ny).unwrap();

    let hitable = vec![ make_sphere(&make_vec3(0.0, 0.0, -1.0), 0.5),
                                    make_sphere(&make_vec3(0.0, -100.5, -1.0), 100.0)];
    let world = make_hitable_list(hitable, 2);
    let camera = standart_camera();

    let bar = ProgressBar::new(ny);
    for j in (0..ny).rev() {
        bar.inc(1);
        for i in 0..nx {
            let mut col = make_vec3(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f32 + rng.gen::<f32>() as f32) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>() as f32) / ny as f32;
                let r = camera.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col += color(&r, &world);
            }
            col /= ns as f32;
            col = col.sqrt();

            write_color(&mut writer, col);
        }
    }
    bar.finish();
 
    Ok(())
}