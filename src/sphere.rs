use crate::{Point3, Ray, HitRecord, Hittable, dot};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn from(center: Point3, radius: f32) -> Self {
        Sphere {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let normal = (ray.at(root) - self.center) / self.radius;
        let record = HitRecord::from(root, ray, normal);
        
        Some(record)
    }
}

