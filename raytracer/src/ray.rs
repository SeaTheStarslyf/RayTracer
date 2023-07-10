use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3,
}

#[derive(Copy, Clone)]
pub struct Hitrecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub num: i32,
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
