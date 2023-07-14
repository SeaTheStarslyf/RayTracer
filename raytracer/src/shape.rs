use crate::material::*;
use crate::ray::*;
use crate::texture::*;
use crate::tool::*;
use crate::vec3::*;
use std::sync::Arc;

pub trait Shape: Sync + Send {
    //    fn getcent(&self) -> Vec3;
    //    fn getradi(&self) -> f64;
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool;
    fn center(&self, time: f64) -> Vec3;
    fn buildbox(&mut self, p0: Vec3, p1: Vec3, texture: Arc<dyn Texture>);
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

pub struct Xyrect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

pub struct Xzrect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

pub struct Yzrect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

pub struct Box {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: Vec<(Arc<dyn Material>, Arc<dyn Shape>)>,
}

pub struct Translate {
    pub ptr: Arc<dyn Shape>,
    pub offset: Vec3,
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
                get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
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
                get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
                rec.num = js;
                return true;
            }
        }
        false
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn buildbox(&mut self, _p0: Vec3, _p1: Vec3, _texture: Arc<dyn Texture>) {}
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
                get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
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
                get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
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
    fn buildbox(&mut self, _p0: Vec3, _p1: Vec3, _texture: Arc<dyn Texture>) {}
}

impl Shape for Xzrect {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        let t = (self.k - r.ori.1) / r.dir.1;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.ori.0 + t * r.dir.0;
        let z = r.ori.2 + t * r.dir.2;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3(0.0, 1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.num = js;
        rec.p = r.at(t);
        true
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn buildbox(&mut self, _p0: Vec3, _p1: Vec3, _texture: Arc<dyn Texture>) {}
}

impl Shape for Yzrect {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        let t = (self.k - r.ori.0) / r.dir.0;
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.ori.1 + t * r.dir.1;
        let z = r.ori.2 + t * r.dir.2;
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.num = js;
        rec.p = r.at(t);
        true
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn buildbox(&mut self, _p0: Vec3, _p1: Vec3, _texture: Arc<dyn Texture>) {}
}

impl Shape for Xyrect {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        let t = (self.k - r.ori.2) / r.dir.2;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.ori.0 + t * r.dir.0;
        let y = r.ori.1 + t * r.dir.1;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        rec.num = js;
        rec.p = r.at(t);
        true
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn buildbox(&mut self, _p0: Vec3, _p1: Vec3, _texture: Arc<dyn Texture>) {}
}

impl Shape for Box {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        let mut ans = t_max;
        for (_haha, i) in (0_i32..).zip(self.sides.iter()) {
            if i.1.gethit(r, rec, t_min, ans, js) {
                ans = rec.t;
            }
        }
        if ans == t_max {
            return false;
        }
        true
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn buildbox(&mut self, p0: Vec3, p1: Vec3, texture: Arc<dyn Texture>) {
        self.sides = Vec::new();
        self.box_min = p0;
        self.box_max = p1;

        let a = Lambertian { albebo: texture };

        let b = Xyrect {
            x0: p0.0,
            x1: p1.0,
            y0: p0.1,
            y1: p1.1,
            k: p1.2,
        };
        self.sides.push((Arc::new(a.clone()), Arc::new(b)));
        let b = Xyrect {
            x0: p0.0,
            x1: p1.0,
            y0: p0.1,
            y1: p1.1,
            k: p0.2,
        };
        self.sides.push((Arc::new(a.clone()), Arc::new(b)));

        let b = Xzrect {
            x0: p0.0,
            x1: p1.0,
            z0: p0.2,
            z1: p1.2,
            k: p1.1,
        };
        self.sides.push((Arc::new(a.clone()), Arc::new(b)));
        let b = Xzrect {
            x0: p0.0,
            x1: p1.0,
            z0: p0.2,
            z1: p1.2,
            k: p0.1,
        };
        self.sides.push((Arc::new(a.clone()), Arc::new(b)));

        let b = Yzrect {
            y0: p0.1,
            y1: p1.1,
            z0: p0.2,
            z1: p1.2,
            k: p1.0,
        };
        self.sides.push((Arc::new(a.clone()), Arc::new(b)));
        let b = Yzrect {
            y0: p0.1,
            y1: p1.1,
            z0: p0.2,
            z1: p1.2,
            k: p0.0,
        };
        self.sides.push((Arc::new(a), Arc::new(b)));
    }
}

impl Shape for Translate {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        let moved_r = Ray {
            ori: reduce(r.ori, self.offset),
            dir: r.dir,
            tm: r.tm,
        };
        if !self.ptr.gethit(moved_r, rec, t_min, t_max, js) {
            return false;
        }
        rec.p = add(rec.p, self.offset);
        rec.set_face_normal(moved_r, rec.normal);
        true
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn buildbox(&mut self, _p0: Vec3, _p1: Vec3, _texture: Arc<dyn Texture>) {}
}
