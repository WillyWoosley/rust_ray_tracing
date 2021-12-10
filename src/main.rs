mod vec3;

pub use vec3::*;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            let r = j as f32 / IMAGE_WIDTH as f32;
            let g = i as f32 / IMAGE_HEIGHT as f32;
            let b: f32 = 0.25;
            
            let ir = (r * 256.).floor() as i32;
            let ig = (g * 256.).floor() as i32;
            let ib = (b * 256.).floor() as i32;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
    let x = Vec3::from(1., 1., 1.);
    let mut y = Vec3::from(2., 2., 2.);
    y /= 10.;

    println!("{:?}", x / 16.);
    println!("{:?}", y);
    
}
