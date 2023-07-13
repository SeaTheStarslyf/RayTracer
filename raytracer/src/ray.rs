use crate::tool::*;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3,
    pub tm: f64,
}

#[derive(Copy, Clone)]
pub struct Hitrecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub num: i32,
}

impl Hitrecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        if dot(r.dir, outward_normal) > 0.0 {
            //inside
            self.normal = multi(outward_normal, -1.0);
            self.front_face = false;
        } else {
            //outside
            self.normal = outward_normal;
            self.front_face = true;
        }
    }
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        Vec3(
            self.ori.0 + self.dir.0 * t,
            self.ori.1 + self.dir.1 * t,
            self.ori.2 + self.dir.2 * t,
        )
    }
}
