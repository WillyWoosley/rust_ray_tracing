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

/// Lambertian material
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

/// Metal material
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn from(albedo: Color, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1. {fuzz} else {1.},
        }
    }
}

impl Material for Metal {
    fn scatter(&self,
               incident: &Ray,
               record: &HitRecord,
               attenuation: &mut Color,
               scattered: &mut Ray) -> bool {
        let reflected = reflect(&unit_vector(incident.direction().clone()), &record.normal);
        *scattered = Ray::from(record.p, reflected + (self.fuzz * Vec3::random_in_unit_sphere()));
        *attenuation = self.albedo;
        
        dot(scattered.direction(), &record.normal) > 0.
    }
}

/// Dielectric material
pub struct Dielectric {
    refraction: f32,
}

impl Dielectric {
    pub fn from(refraction: f32) -> Self {
        Dielectric {refraction}
    }
}

impl Material for Dielectric {
    fn scatter(&self,
               incident: &Ray, 
               record: &HitRecord, 
               attenuation: &mut Color, 
               scattered: &mut Ray) -> bool {
        let refraction_ratio = if record.front_face {1./self.refraction} 
                               else {self.refraction};
        let unit_direction = unit_vector(*incident.direction());
        let refracted = refract(unit_direction, record.normal, refraction_ratio);

        *attenuation = Color::from(1., 1., 1.);
        *scattered = Ray::from(record.p, refracted);
        
        true
    }
}
