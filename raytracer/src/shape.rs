use crate::ray::*;
use crate::tool::*;
use crate::vec3::*;

pub trait Shape: Sync + Send {
    //    fn getcent(&self) -> Vec3;
    //    fn getradi(&self) -> f64;
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool;
    fn center(&self, time: f64) -> Vec3;
}

pub struct Sphere {
    pub cent: Vec3,
    pub radi: f64,
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
}

impl Shape for Sphere {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        let center: Vec3 = self.cent;
        let radius: f64 = self.radi;
        let oc = Vec3(r.ori.0 - center.0, r.ori.1 - center.1, r.ori.2 - center.2);
        let a: f64 = dot(r.dir, r.dir);
        let half_b: f64 = dot(oc, r.dir);
        let c: f64 = dot(oc, oc) - radius * radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            let t1: f64 = (-half_b - discriminant.sqrt()) / a; //注意取近时容易出错
            if t1 < t_max && t1 > t_min {
                //0.001
                rec.t = t1;
                rec.p = r.at(rec.t);
                let outward_normal = divis(reduce(rec.p, center), radius);
                rec.set_face_normal(r, outward_normal);
                rec.num = js;
                return true;
            }
            let t1: f64 = (-half_b + discriminant.sqrt()) / a;
            if t1 < t_max && t1 > t_min {
                //0.001
                rec.t = t1;
                rec.p = r.at(rec.t);
                let outward_normal = divis(reduce(rec.p, center), radius);
                rec.set_face_normal(r, outward_normal);
                rec.num = js;
                return true;
            }
        }
        false
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    /*    fn getcent(&self) -> Vec3 {
        self.cent
    }
    fn getradi(&self) -> f64 {
        self.radi
    }*/
}

impl Shape for MovingSphere {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        //        let center: Vec3 = self.cent;
        //       let radius: f64 = self.radi;
        let oc = reduce(r.ori, self.center(r.tm));
        let a: f64 = dot(r.dir, r.dir);
        let half_b: f64 = dot(oc, r.dir);
        let c: f64 = dot(oc, oc) - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant >= 0.0 {
            let t1: f64 = (-half_b - discriminant.sqrt()) / a; //注意取近时容易出错
            if t1 < t_max && t1 > t_min {
                //0.001
                rec.t = t1;
                rec.p = r.at(rec.t);
                let outward_normal = divis(reduce(rec.p, self.center(r.tm)), self.radius);
                rec.set_face_normal(r, outward_normal);
                rec.num = js;
                return true;
            }
            let t1: f64 = (-half_b + discriminant.sqrt()) / a;
            if t1 < t_max && t1 > t_min {
                //0.001
                rec.t = t1;
                rec.p = r.at(rec.t);
                let outward_normal = divis(reduce(rec.p, self.center(r.tm)), self.radius);
                rec.set_face_normal(r, outward_normal);
                rec.num = js;
                return true;
            }
        }
        false
    }
    fn center(&self, time: f64) -> Vec3 {
        add(
            self.center0,
            multi(
                reduce(self.center1, self.center0),
                (time - self.time0) / (self.time1 - self.time0),
            ),
        )
    }
}
