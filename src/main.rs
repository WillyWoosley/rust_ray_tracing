mod vec3;
mod color;
mod ray; 

use vec3::*;
use color::*;
use ray::*;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", i);
        for j in 0..IMAGE_WIDTH {
            let pixel_color = Vec3::from(j as f32 / (IMAGE_WIDTH-1) as f32, 
                                         i as f32 / (IMAGE_HEIGHT-1) as f32, 
                                         0.25);
            print_color(&pixel_color);
        }
    }

    println!("Done.");
}
