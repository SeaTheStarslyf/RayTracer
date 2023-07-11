use crate::ray::*;
use crate::tool::*;
use crate::vec3::*;

pub trait Shape {
    fn getcent(&self) -> Vec3;
    fn getradi(&self) -> f64;
    fn gethit(&self, r: Ray) -> f64;
}

pub struct Sphere {
    pub cent: Vec3,
    pub radi: f64,
}

impl Shape for Sphere {
    fn gethit(&self, r: Ray) -> f64 {
        let mut ans: f64 = 0x10000000 as f64;
        let center: Vec3 = self.cent;
        let radius: f64 = self.radi;
        let oc = Vec3(r.ori.0 - center.0, r.ori.1 - center.1, r.ori.2 - center.2);
        let a: f64 = dot(r.dir, r.dir);
        let b: f64 = 2.0 * dot(oc, r.dir);
        let c: f64 = dot(oc, oc) - radius * radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t: f64 = (-b - discriminant.sqrt()) / (2.0 * a); //注意取近时容易出错
            if t < ans && t > 0.001 {
                ans = t;
            } else {
                let t: f64 = (-b + discriminant.sqrt()) / (2.0 * a);
                if t < ans && t > 0.001 {
                    ans = t;
                }
            }
        }
        if ans == (0x10000000 as f64) {
            -1.0
        } else {
            ans
        }
    }
    fn getcent(&self) -> Vec3 {
        self.cent
    }
    fn getradi(&self) -> f64 {
        self.radi
    }
}
