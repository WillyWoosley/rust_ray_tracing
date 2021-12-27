use crate::{Ray, Point3, Vec3, dot, Material};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn from(t: f32, ray: &Ray, normal: Vec3, material: &'a dyn Material) -> Self {
        let mut record = HitRecord {
            t,
            normal,
            material,
            p: ray.at(t),
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
    fn hit<T: Material>(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<T>>;
}

pub struct HittableList<T> {
    objects: Vec<Box<T>>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        HittableList {objects: Vec::new()}
    }

    pub fn push(&mut self, item: T) {
        self.objects.push(Box::new(item));
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit<M: Material>(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<M>> {
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
