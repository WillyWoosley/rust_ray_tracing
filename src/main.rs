use std::io::{Write, stdout};

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            eprintln!("\rScanlines remaining: {}", i);
            stdout().flush();

            let r = j as f32 / IMAGE_WIDTH as f32;
            let g = i as f32 / IMAGE_HEIGHT as f32;
            let b: f32 = 0.25;
            
            let ir = (r * 256.).floor() as i32;
            let ig = (g * 256.).floor() as i32;
            let ib = (b * 256.).floor() as i32;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
