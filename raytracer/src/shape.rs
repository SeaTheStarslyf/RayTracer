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
    fn buildrotate(&mut self, p: Arc<dyn Shape>, angle: f64);
    fn getmin(&self) -> Vec3;
    fn getmax(&self) -> Vec3;
}

#[derive(Clone)]
pub struct Sphere {
    pub cent: Vec3,
    pub radi: f64,
}

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
}

#[derive(Clone)]
pub struct Xyrect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

#[derive(Clone)]
pub struct Xzrect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

#[derive(Clone)]
pub struct Yzrect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

#[derive(Clone)]
pub struct Box {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: Vec<(Arc<dyn Material>, Arc<dyn Shape>)>,
    pub trans: Arc<dyn Shape>,
    pub rotat: Arc<dyn Shape>,
}

#[derive(Clone)]
pub struct Translate {
    pub ptr: Arc<dyn Shape>,
    pub offset: Vec3,
}

#[derive(Clone)]
pub struct Rotatey {
    pub ptr: Arc<dyn Shape>,
    pub sin_theta: f64,
    pub cos_theta: f64,
}

#[derive(Clone)]
pub struct Constantmedium {
    pub boundary: Arc<dyn Shape>,
    pub phase_function: Arc<dyn Material>,
    pub neg_inv_density: f64,
}

#[derive(Clone)]
pub struct Triangle1 {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
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
    fn buildrotate(&mut self, _p: Arc<dyn Shape>, _angle: f64) {}
    fn getmax(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn getmin(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
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
    fn buildrotate(&mut self, _p: Arc<dyn Shape>, _angle: f64) {}
    fn getmax(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn getmin(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
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
    fn buildrotate(&mut self, _p: Arc<dyn Shape>, _angle: f64) {}
    fn getmax(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn getmin(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
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
    fn buildrotate(&mut self, _p: Arc<dyn Shape>, _angle: f64) {}
    fn getmax(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn getmin(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
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
    fn buildrotate(&mut self, _p: Arc<dyn Shape>, _angle: f64) {}
    fn getmax(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn getmin(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Shape for Box {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        let mut ans = t_max;
        for (_haha, i) in (0_i32..).zip(self.sides.iter()) {
            //            let _ifhit = i.1.gethit(r, rec, t_min, ans, js);
            //            let _ifhit = self.rotat.gethit(r, rec, t_min, ans, js);
            //           let ifhit = self.trans.gethit(r, rec, t_min, ans, js);
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
    fn buildrotate(&mut self, _p: Arc<dyn Shape>, _angle: f64) {}
    fn getmax(&self) -> Vec3 {
        self.box_max
    }
    fn getmin(&self) -> Vec3 {
        self.box_min
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
    fn buildrotate(&mut self, _p: Arc<dyn Shape>, _angle: f64) {}
    fn getmax(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn getmin(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Shape for Rotatey {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        let mut origin = r.ori;
        let mut direction = r.dir;

        origin.0 = self.cos_theta * r.ori.0 - self.sin_theta * r.ori.2;
        origin.2 = self.sin_theta * r.ori.0 + self.cos_theta * r.ori.2;

        direction.0 = self.cos_theta * r.dir.0 - self.sin_theta * r.dir.2;
        direction.2 = self.sin_theta * r.dir.0 + self.cos_theta * r.dir.2;

        let rotated_r = Ray {
            ori: origin,
            dir: direction,
            tm: r.tm,
        };

        if !self.ptr.gethit(rotated_r, rec, t_min, t_max, js) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.0 = self.cos_theta * rec.p.0 + self.sin_theta * rec.p.2;
        p.2 = -self.sin_theta * rec.p.0 + self.cos_theta * rec.p.2;

        normal.0 = self.cos_theta * rec.normal.0 + self.sin_theta * rec.normal.2;
        normal.2 = -self.sin_theta * rec.normal.0 + self.cos_theta * rec.normal.2;

        rec.p = p;
        rec.set_face_normal(rotated_r, normal);

        true
    }
    fn buildrotate(&mut self, p: Arc<dyn Shape>, angle: f64) {
        let randians = degrees_to_radians(angle);
        self.sin_theta = randians.sin();
        self.cos_theta = randians.cos();

        let mut min = Vec3(
            100000000000000000.0,
            100000000000000000.0,
            100000000000000000.0,
        );
        let mut max = Vec3(
            -100000000000000000.0,
            -100000000000000000.0,
            -100000000000000000.0,
        );

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * p.getmax().0 + (1 - i) as f64 * p.getmin().0;
                    let y = j as f64 * p.getmax().1 + (1 - j) as f64 * p.getmin().1;
                    let z = k as f64 * p.getmax().2 + (1 - k) as f64 * p.getmin().2;

                    let newx = self.cos_theta * x + self.sin_theta * z;
                    let newz = -self.sin_theta * x + self.cos_theta * z;

                    let tester = Vec3(newx, y, newz);

                    min.0 = f64::min(min.0, tester.0);
                    max.0 = f64::max(max.0, tester.0);
                    min.1 = f64::min(min.1, tester.1);
                    max.1 = f64::max(max.1, tester.1);
                    min.2 = f64::min(min.2, tester.2);
                    max.2 = f64::max(max.2, tester.2);
                }
            }
        }
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn buildbox(&mut self, _p0: Vec3, _p1: Vec3, _texture: Arc<dyn Texture>) {}
    fn getmax(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn getmin(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Shape for Constantmedium {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        //let enableDebug = false;
        //let debugging = enableDebug && random_double(0.0, 1.0) < 0.00001;

        let mut rec1 = Hitrecord {
            p: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: -1.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            num: -1,
        };
        let mut rec2 = Hitrecord {
            p: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: -1.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            num: -1,
        };

        let maxn = 0x10000000 as f64;
        let minn = -maxn;
        if !self
            .boundary
            .gethit(r, &mut rec1, minn, 0x10000000 as f64, js)
        {
            return false;
        }

        if !self
            .boundary
            .gethit(r, &mut rec2, rec1.t + 0.0001, 0x10000000 as f64, js)
        {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = dot(r.dir, r.dir).sqrt();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double(0.0, 1.0).ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.num = js;

        true
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn buildbox(&mut self, _p0: Vec3, _p1: Vec3, _texture: Arc<dyn Texture>) {}
    fn buildrotate(&mut self, _p: Arc<dyn Shape>, _angle: f64) {}
    fn getmax(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn getmin(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Shape for Triangle1 {
    fn gethit(&self, r: Ray, rec: &mut Hitrecord, t_min: f64, t_max: f64, js: i32) -> bool {
        let edge1 = reduce(self.v1, self.v0);
        let edge2 = reduce(self.v2, self.v0);

        let h = cross(r.dir, edge2);
        let a = dot(edge1, h);

        if a > -t_min && a < t_min {
            return false;
        }

        let f = 1.0 / a;
        let s = reduce(r.ori, self.v0);
        let u = f * dot(s, h);

        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = cross(s, edge1);
        let v = f * dot(r.dir, q);

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f * dot(edge2, q);

        if t > t_min && t < t_max {
            let p = r.at(t);
            let nor = cross(edge1, edge2);
            let normal = divis(nor, dot(nor, nor).sqrt());

            rec.t = t;
            rec.p = p;
            rec.num = js;
            rec.set_face_normal(r, normal);
            let v0p = reduce(p, self.v0);
            let denom = dot(edge1, edge1) * dot(edge2, edge2) - dot(edge1, edge2).powi(2);
            rec.u =
                (dot(v0p, edge1) * dot(edge2, edge2) - dot(v0p, edge2) * dot(edge1, edge2)) / denom;
            rec.v =
                (dot(v0p, edge2) * dot(edge1, edge1) - dot(v0p, edge1) * dot(edge1, edge2)) / denom;

            return true;
        }
        false
    }
    fn center(&self, _time: f64) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn buildbox(&mut self, _p0: Vec3, _p1: Vec3, _texture: Arc<dyn Texture>) {}
    fn buildrotate(&mut self, _p: Arc<dyn Shape>, _angle: f64) {}
    fn getmax(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    fn getmin(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}
