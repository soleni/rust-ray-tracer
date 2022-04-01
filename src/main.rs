#![allow(unused_imports)]

use std::io;
use std::fs::File;
use std::io::Write;

mod vec3;
mod ray;

use vec3::*;
use ray::*;

use indicatif::ProgressBar;

fn color(ray : &Ray) -> Vec3 {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    make_vec3(1.0, 1.0,1.0) * (1.0 - t) + make_vec3(0.5, 0.7, 1.0) * t
}

fn main() -> io::Result<()> {
    let mut writer = File::create("output.ppm").unwrap();

    let nx = 200;
    let ny = 100;

    let lower_left_corner = make_vec3(-2.0, -1.0, -1.0);
    let horizontal = make_vec3(4.0, 0.0, 0.0);
    let vertical = make_vec3(0.0, 2.0, 0.0);
    let origin = make_vec3(0.0, 0.0, 0.0);

    write!(&mut writer, "P3\n{} {}\n255\n",nx, ny).unwrap();

    let bar = ProgressBar::new(ny*nx);
    for j in (0..ny).rev() {
        for i in 0..nx {
           // bar.inc(1);
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let ray = Ray{a: origin, b: lower_left_corner + horizontal * u + vertical * v};
            let col = color(&ray);

            write_color(&mut writer, col);
        }
    }
    bar.finish();
 
    Ok(())
}