use crate::vec3::*;
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    ll_corner: Vec3,
    hori: Vec3,
    vert: Vec3,
}

impl Camera {
    pub fn new(view_from: Point3, view_at: Point3, view_up: Vec3, vfov: f32,
               aspect_ratio: f32) -> Self {
        let h = (vfov.to_radians() / 2.).tan();
        let view_height = 2. * h;
        let view_width = aspect_ratio * view_height;
        
        let w = unit_vector(view_from - view_at);
        let u = unit_vector(cross(&view_up, &w));
        let v = cross(&w, &u);

        let origin = view_from;
        let hori = view_width * u;
        let vert = view_height * v;
        Camera {
            origin,
            hori,
            vert,
            ll_corner: origin - hori/2. - vert/2. - w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::from(self.origin, 
                  self.ll_corner + u * self.hori + v * self.vert - self.origin
        )
    }
}
