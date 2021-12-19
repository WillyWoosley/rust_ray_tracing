use crate::{Ray, Point3, Vec3, dot};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn from(t: f32, ray: &Ray, normal: Vec3) -> Self {
        let mut record = HitRecord {
            t: t,
            p: ray.at(t),
            normal: normal,
            front_face: false,
        };
        record.set_face_normal(ray);

        record
    }

    pub fn set_face_normal(&mut self, ray: &Ray) {
        self.front_face = dot(ray.direction(), &self.normal) < 0.;
        self.normal = if self.front_face {self.normal} else {-self.normal};
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

struct HittableList<T: Hittable> {
    objects: Vec<Box<T>>,
}

impl<T: Hittable> HittableList<T> {
    pub fn add(&mut self, item: T) {
        self.objects.push(Box::new(item));
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_rec = None;
        let mut closest = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest) {
                closest = rec.t;
                hit_rec = Some(rec);
            }
        }

        hit_rec
    }
}
