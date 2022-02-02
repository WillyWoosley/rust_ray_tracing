use std::env;
use std::process;
use std::fs;
use std::sync::Arc;

use rand::prelude::*;
use rayon::prelude::*;

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

struct ImageArgs {
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    filename: String,
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let mat_ground = Arc::new(Lambertian::from(Color::from(0.5, 0.5, 0.5)));
    world.push(Sphere::from(Point3::from(0., -1000., 0.), 1000., Arc::clone(&mat_ground)));

    let mut rng = rand::thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let mat_type: f32 = rng.gen();
            let center = Point3::from(i as f32 + 0.9 * rng.gen::<f32>(), 
                                      0.2, 
                                      j as f32 + 0.9 * rng.gen::<f32>());
            
            if (center - Point3::from(4., 0.2, 0.)).length() > 0.9 {
                if mat_type < 0.8 { // Make Diffuse sphere
                    let mat_sphere = Arc::new(Lambertian::from(
                                                Color::from(rng.gen::<f32>(), 
                                                            rng.gen::<f32>(), 
                                                            rng.gen::<f32>())));
                    world.push(Sphere::from(center, 0.2, Arc::clone(&mat_sphere)));
                } else if mat_type < 0.95 { // Make Metal sphere
                    let mat_sphere = Arc::new(Metal::from(
                                                Color::from(rng.gen_range(0.5..1.),
                                                            rng.gen_range(0.5..1.),
                                                            rng.gen_range(0.5..1.)),
                                                rng.gen_range(0.0..0.5)));
                    world.push(Sphere::from(center, 0.2, Arc::clone(&mat_sphere)));
                } else { // Make Glass sphere
                    let mat_sphere = Arc::new(Dielectric::from(1.5));
                    world.push(Sphere::from(center, 0.2, Arc::clone(&mat_sphere)));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::from(1.5));
    let mat2 = Arc::new(Lambertian::from(Color::from(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::from(Color::from(0.7, 0.6, 0.5), 0.));
    world.push(Sphere::from(Point3::from(0., 1., 0.), 1., Arc::clone(&mat1)));
    world.push(Sphere::from(Point3::from(-4., 1., 0.), 1., Arc::clone(&mat2)));
    world.push(Sphere::from(Point3::from(4., 1., 0.), 1., Arc::clone(&mat3)));

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

// Logic for pasring command line image generation arguments
fn parse_args(args: &Vec<String>) -> ImageArgs {
    // Basic help message
    fn help() {
        eprintln!("expected usage: rust_ray_trace <image width> <samples> <maximum depth> \
                    <filename>");
    }
    
    // Error and exit if not enough args
    if args.len() < 5 {
        if args.len() > 1 && args[1] == "help" {
            help();
            process::exit(0);
        } else {
            eprintln!("Error parsing arguments: Expected 4 arguments, received {}",
                        args.len() - 1);
            help();
            process::exit(1);
        }
    }

    // Parse args, error and exit if not properly formatted
    let width: u32 = match args[1].parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("Error parsing arguments: Expected a positive integer, \
                        received \"{}\"", args[1]);
            help();
            process::exit(1);
        },
    };
    let height = (width as f32 / ASPECT_RATIO) as u32;
    let samples: u32 = match args[2].parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("Error parsing arguments: Expected a positive integer, \
                        received \"{}\"", args[2]);
            help();
            process::exit(1);
        },
    };
    let max_depth: u32 = match args[3].parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("Error parsing arguments: Expected a positive integer, \
                        received \"{}\"", args[3]);
            help();
            process::exit(1);
        },
    };
    let filename = args[4].clone();

    // Error and exit if args are not "sane" size"
    if width < 100 {
        eprintln!("Error parsing arguments: image width should be at least 100 pixels");
        help();
        process::exit(1);
    }
    if samples < 1 {
        eprintln!("Error parsing arguments: samples should be at least 1");
        help();
        process::exit(1);
    }
    if max_depth < 1 {
        eprintln!("Error parsing arguments: maximum depth should be at least 1");
        help();
        process::exit(1);
    }

    ImageArgs {
        width,
        height,
        samples,
        max_depth,
        filename,
    }
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let image_args = parse_args(&args);

    // Create randomized scene
    eprintln!("Beginning {} x {} image with {} samples and a maximum depth of {}...",
                image_args.width, image_args.height, image_args.samples,
                image_args.max_depth);
    let world = random_scene();

    // Setup Camera
    let view_from = Point3::from(13., 2., 3.);
    let view_at = Point3::from(0., 0., 0.);
    let camera = Camera::new(view_from, view_at, Vec3::from(0., 1., 0.), 20.,
                             ASPECT_RATIO, 0.1, 10.);

    // Actually generate image data
    let ppm_header = format!("P3\n{} {}\n255\n", // 255 denotes maximum color value
                                image_args.width, image_args.height);

    eprintln!("Generating...");
    let image_data =
        (0..image_args.height).into_par_iter().rev().map(|i| {
            (0..image_args.width).into_par_iter().map(|j| {
                let mut rng = rand::thread_rng();
                let mut pixel_color = Color::new();
                for _ in 0..image_args.samples {
                    let r1: f32 = rng.gen();
                    let r2: f32 = rng.gen();
                    let u = (j as f32 + r1) / (image_args.width - 1) as f32;
                    let v = (i as f32 + r2) / (image_args.height - 1) as f32;

                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, image_args.max_depth);
                }
                format_color(&pixel_color, image_args.samples) 
            }).collect::<Vec<String>>().join("")
        }).collect::<Vec<String>>().join("");

    // Write final generated image
    let final_pic = format!("{}{}", ppm_header, image_data);
    match fs::write(format!("{}.ppm", image_args.filename), final_pic) {
        Ok(_) => eprintln!(
                    "Image generated successfully! Output written to \"{}.ppm\"",
                        image_args.filename),
        Err(_) => eprintln!("Error writing image data"),
    }
}

