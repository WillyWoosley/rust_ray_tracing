mod vec3;
mod color;
mod ray; 

use vec3::*;
use color::*;
use ray::*;

const ASPECT_RATIO: f32 = 16./9.;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> bool {
    let oc = *ray.origin() - *center;
    let a = dot(ray.direction(), ray.direction());
    let b = 2. * dot(&oc, ray.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    
    discriminant > 0.
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::from(0., 0., -1.), 0.5, ray) {
        return Color::from(1., 0., 0.);
    }

    let unit_direction = unit_vector(*ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    
    let origin = Point3::new();
    let horizontal = Vec3::from(viewport_width, 0., 0.);
    let vertical = Vec3::from(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3::from(0., 0., focal_length);

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", i);
        for j in 0..IMAGE_WIDTH {
            let u = j as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = i as f32 / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray::from(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            print_color(&ray_color(&r));
        }
    }

    eprintln!("Done.");
}
