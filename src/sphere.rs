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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }
        
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(record.t);
        
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        
        true
    }
}

