mod vec3;
mod color;
mod ray; 
mod hittable;
mod sphere;
mod camera;

use vec3::*;
use color::*;
use ray::*;
use hittable::*;
use sphere::*;
use camera::*;

use rand::prelude::*;

const ASPECT_RATIO: f32 = 16./9.;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES: u32 = 100;

fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
    if let Some(record) = world.hit(ray, 0., f32::INFINITY) {
        return 0.5 * (record.normal + Color::from(1., 1., 1.));
    }

    let unit_direction = unit_vector(*ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    let mut world = HittableList::new();    
    world.push(Sphere::from(Point3::from(0., 0., -1.), 0.5));
    world.push(Sphere::from(Point3::from(0., -100.5, -1.), 100.));

    let camera = Camera::new();

    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", i);
        for j in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();
            for s in 0..SAMPLES {
                let r1: f32 = rng.gen();
                let r2: f32 = rng.gen();
                let u = (j as f32 + r1) / (IMAGE_WIDTH - 1) as f32;
                let v = (i as f32 + r2) / (IMAGE_HEIGHT - 1) as f32;
                
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world)
            }
            print_color(&pixel_color, SAMPLES);
        }
    }

    eprintln!("Done.");
}
