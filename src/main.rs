use std::io;
use std::fs::File;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut writer = File::create("output.ppm").unwrap();

    let nx = 200;
    let ny = 100;

    write!(&mut writer, "P3\n{} {}\n255\n",nx, ny).unwrap();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b: f32 = 0.2;

            let ir = (r * 255.99) as i32;
            let ig = (g * 255.99) as i32;
            let ib = (b * 255.99) as i32;

            writeln!(&mut writer, "{} {} {}", ir, ig, ib).unwrap();
        }
    }

    Ok(())
}