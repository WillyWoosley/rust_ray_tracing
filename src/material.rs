use rand::random;

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
        let reflected = reflect(&unit_vector(*incident.direction()), &record.normal);
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

    // Schlick approximation for reflectance
    fn reflectance(cos: f32, ref_idx: f32) -> f32 {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0) * (1. - cos).powi(5)
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

        let cos_theta = dot(&-unit_direction, &record.normal).min(1.); 
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();

        let direction = 
            if refraction_ratio * sin_theta > 1. ||
               Self::reflectance(cos_theta, refraction_ratio) > random::<f32>() {
                reflect(&unit_direction, &record.normal)
            } else {
                refract(unit_direction, record.normal, refraction_ratio)
            };

        *attenuation = Color::from(1., 1., 1.);
        *scattered = Ray::from(record.p, direction);

        true
    }
}

