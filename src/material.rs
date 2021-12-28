use crate::ray::Ray;
use crate::vec3::*;
use crate::hittable::HitRecord;

pub trait Material {
    fn scatter(&self,
               incident: &Ray, 
               record: &HitRecord, 
               attenuation: &mut Color, 
               scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn from(albedo: Color) -> Self {
        Lambertian {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, 
               incident: &Ray, 
               record: &HitRecord, 
               attenuation: &mut Color, 
               scattered: &mut Ray) -> bool {
        let mut scatter_dir = record.normal + Vec3::random_unit_vector();
        
        // Catch degenerate scatter direction
        if scatter_dir.near_zero() {
            scatter_dir = record.normal;
        }

        *scattered = Ray::from(record.p, scatter_dir);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn from(albedo: Color) -> Self {
        Metal {albedo}
    }
}

impl Material for Metal {
    fn scatter(&self,
               incident: &Ray,
               record: &HitRecord,
               attenuation: &mut Color,
               scattered: &mut Ray) -> bool {
        let reflected = reflect(&unit_vector(incident.direction().clone()), &record.normal);
        *scattered = Ray::from(record.p, reflected);
        *attenuation = self.albedo;
        
        dot(scattered.direction(), &record.normal) > 0.
    }
}
