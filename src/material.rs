use crate::ray::Ray;
use crate::vec3::Color;
use crate::hittable::HitRecord;

pub trait Material {
    fn scatter(&self,
               incident: &Ray, 
               record: &HitRecord, 
               attenuation: &Color, 
               scattered: &Ray) -> bool;
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
               attenuation: &Color, 
               scattered: &Ray) -> bool {
        true
    }
}
