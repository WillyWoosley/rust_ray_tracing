use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3, 
    dir: Vec3, 
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            orig: Point3::default(),
            dir: Vec3::default(),
        }
    }

    pub fn from(orig: Point3, dir: Vec3) -> Self {
        Ray {
            orig,
            dir,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}

