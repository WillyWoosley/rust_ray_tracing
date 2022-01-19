mod vec3;
mod color;
mod ray; 
mod hittable;
mod sphere;
mod camera;
mod material;

use vec3::*;
use color::*;
use ray::*;
use hittable::*;
use sphere::*;
use camera::*;
use material::*;

use rand::prelude::*;

const ASPECT_RATIO: f32 = 16./9.;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn ray_color<T: Hittable>(ray: &Ray, world: &T, depth: u32) -> Color {
    // Check if we've exceeded the 'bounce limit'
    if depth == 0 {
        return Color::new();
    }

    if let Some(record) = world.hit(ray, 0.001, f32::INFINITY) {
        let mut scattered = Ray::new();
        let mut attenuation = Color::new();
        
        if record.material.scatter(ray, &record, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1);
        } else {
            return Color::new();
        }
    }

    let unit_direction = unit_vector(*ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    // World creation
    let mut world = HittableList::new();
    
    let mat_ground = Lambertian::from(Color::from(0.8, 0.8, 0.));
    let mat_center = Lambertian::from(Color::from(0.1, 0.2, 0.5));
    let mat_left = Dielectric::from(1.5);
    let mat_left_2 = Dielectric::from(1.5);
    let mat_right = Metal::from(Color::from(0.8, 0.6, 0.2), 0.);
    
    world.push(Sphere::from(Point3::from(0., -100.5, -1.), 100., mat_ground));
    world.push(Sphere::from(Point3::from(0., 0., -1.), 0.5, mat_center));
    world.push(Sphere::from(Point3::from(-1., 0., -1.), 0.5, mat_left));
    world.push(Sphere::from(Point3::from(-1., 0., -1.), -0.4, mat_left_2));
    world.push(Sphere::from(Point3::from(1., 0., -1.), 0.5, mat_right));

    let camera = Camera::new();

    let mut rng = rand::thread_rng();

    // Rendering
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", i);
        for j in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();
            for _ in 0..SAMPLES {
                let r1: f32 = rng.gen();
                let r2: f32 = rng.gen();
                let u = (j as f32 + r1) / (IMAGE_WIDTH - 1) as f32;
                let v = (i as f32 + r2) / (IMAGE_HEIGHT - 1) as f32;
                
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH)
            }
            print_color(&pixel_color, SAMPLES);
        }
    }

    eprintln!("Done.");
}
