use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    ll_corner: Vec3,
    hori: Vec3,
    vert: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16. / 9.;
        let view_height = 2.;
        let view_width = aspect_ratio * view_height;
        let focal_length = 1.;
       
        let origin = Point3::from(0., 0., 0.);
        let hori = Vec3::from(aspect_ratio * view_height, 0., 0.);
        let vert = Vec3::from(0., view_height, 0.);

        Camera {
            origin,
            hori,
            vert,
            ll_corner: origin - hori/2. - vert/2. - Vec3::from(0., 0., focal_length),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::from(self.origin, 
                  self.ll_corner + u * self.hori + v * self.vert - self.origin
        )
    }
}
