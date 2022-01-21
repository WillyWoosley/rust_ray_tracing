use crate::vec3::*;
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    ll_corner: Vec3,
    hori: Vec3,
    vert: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(view_from: Point3, view_at: Point3, view_up: Vec3, vfov: f32,
               aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        let h = (vfov.to_radians() / 2.).tan();
        let view_height = 2. * h;
        let view_width = aspect_ratio * view_height;
        
        let w = unit_vector(view_from - view_at);
        let u = unit_vector(cross(&view_up, &w));
        let v = cross(&w, &u);
        let origin = view_from;
        let hori = focus_dist * view_width * u;
        let vert = focus_dist * view_height * v;
        let ll_corner = origin - hori/2. - vert/2. - focus_dist * w;
        let lens_radius = aperture / 2.;

        Camera {
            w,
            u,
            v,
            origin,
            hori,
            vert,
            ll_corner,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::from(self.origin + offset, 
                  self.ll_corner + u * self.hori + v * self.vert - self.origin - offset
        )
    }
}
