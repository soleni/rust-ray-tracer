#![allow(unused_imports)]

use std::io;
use std::fs::File;
use std::io::Write;

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn main() -> io::Result<()> {
    let mut writer = File::create("output.ppm").unwrap();

    let nx = 200;
    let ny = 100;

    write!(&mut writer, "P3\n{} {}\n255\n",nx, ny).unwrap();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let v = Vec3{x: i as f32 / nx as f32, y: j as f32 / ny as f32, z: 0.2};

            let ir = (v[0] * 255.99) as i32;
            let ig = (v[1] * 255.99) as i32;
            let ib = (v[2] * 255.99) as i32;

            writeln!(&mut writer, "{} {} {}", ir, ig, ib).unwrap();
        }
    }

    Ok(())
}