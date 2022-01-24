use std::rc::Rc;

use rand::prelude::*;

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

const ASPECT_RATIO: f32 = 3./2.;
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES: u32 = 500;
const MAX_DEPTH: u32 = 50;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let mat_ground = Rc::new(Lambertian::from(Color::from(0.5, 0.5, 0.5)));
    world.push(Sphere::from(Point3::from(0., -1000., 0.), 1000., Rc::clone(&mat_ground)));

    let mut rng = rand::thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let mat_type: f32 = rng.gen();
            let center = Point3::from(i as f32 + 0.9 * rng.gen::<f32>(), 
                                      0.2, 
                                      j as f32 + 0.9 * rng.gen::<f32>());
            
            if (center - Point3::from(4., 0.2, 0.)).length() > 0.9 {
                if mat_type < 0.8 { // Make Diffuse sphere
                    let mat_sphere = Rc::new(Lambertian::from(
                                                Color::from(rng.gen::<f32>(), 
                                                            rng.gen::<f32>(), 
                                                            rng.gen::<f32>())));
                    world.push(Sphere::from(center, 0.2, Rc::clone(&mat_sphere)));
                } else if mat_type < 0.95 { // Make Metal sphere
                    let mat_sphere = Rc::new(Metal::from(
                                                Color::from(rng.gen_range(0.5..1.),
                                                            rng.gen_range(0.5..1.),
                                                            rng.gen_range(0.5..1.)),
                                                rng.gen_range(0.0..0.5)));
                    world.push(Sphere::from(center, 0.2, Rc::clone(&mat_sphere)));
                } else { // Make Glass sphere
                    let mat_sphere = Rc::new(Dielectric::from(1.5));
                    world.push(Sphere::from(center, 0.2, Rc::clone(&mat_sphere)));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric::from(1.5));
    let mat2 = Rc::new(Lambertian::from(Color::from(0.4, 0.2, 0.1)));
    let mat3 = Rc::new(Metal::from(Color::from(0.7, 0.6, 0.5), 0.));
    world.push(Sphere::from(Point3::from(0., 1., 0.), 1., Rc::clone(&mat1)));
    world.push(Sphere::from(Point3::from(-4., 1., 0.), 1., Rc::clone(&mat2)));
    world.push(Sphere::from(Point3::from(4., 1., 0.), 1., Rc::clone(&mat3)));

    world
}

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
    let world = random_scene();

    // Camera
    let view_from = Point3::from(13., 2., 3.);
    let view_at = Point3::from(0., 0., 0.);
    let camera = Camera::new(view_from, view_at, Vec3::from(0., 1., 0.), 20.,
                             ASPECT_RATIO, 0.1, 10.);

    // Rendering
    let mut rng = rand::thread_rng();

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
