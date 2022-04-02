#![allow(unused_imports)]

use std::io;
use std::fs::File;
use std::io::Write;

mod vec3;
mod ray;
mod sphere;
mod hitable;
mod hitablelist;

use vec3::*;
use ray::*;
use sphere::*;
use hitable::*;
use hitablelist::*;

use indicatif::ProgressBar;

fn color<T: Hitable>(ray : &Ray, world: &HitableList<T>) -> Vec3 {
    let mut rec = HitRecord{t: 0.0, p: unit_vector(), normal: unit_vector()};
    if world.hit(ray, 0.0, f32::MAX, &mut rec) {
        return make_vec3(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0) * 0.5; 
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return make_vec3(1.0, 1.0,1.0) * (1.0 - t) + make_vec3(0.2, 0.5, 1.0) * t;
    }
}

fn main() -> io::Result<()> {
    let mut writer = File::create("output.ppm").unwrap();

    let nx = 400;
    let ny = 200;
    write!(&mut writer, "P3\n{} {}\n255\n",nx, ny).unwrap();

    let lower_left_corner = make_vec3(-2.0, -1.0, -1.0);
    let horizontal = make_vec3(4.0, 0.0, 0.0);
    let vertical = make_vec3(0.0, 2.0, 0.0);
    let origin = make_vec3(0.0, 0.0, 0.0);

    let hitable = vec![ make_sphere(&make_vec3(0.0, 0.0, -1.0), 0.5),
                                    make_sphere(&make_vec3(0.0, -100.5, -1.0), 100.0)];
    let world = make_hitable_list(hitable, 2);

    //let bar = ProgressBar::new(ny*nx);
    for j in (0..ny).rev() {
        for i in 0..nx {
            //bar.inc(1);
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let ray = Ray{a: origin, b: lower_left_corner + horizontal * u + vertical * v};
            let col = color(&ray, &world);

            write_color(&mut writer, col);
        }
    }
    //bar.finish();
 
    Ok(())
}