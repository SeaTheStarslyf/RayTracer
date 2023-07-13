//use crate::ray::*;
use crate::tool::*;
use crate::vec3::*;
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct Solidcolor {
    pub color: Vec3,
}

pub struct Checkertexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl Texture for Solidcolor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let _haha = multi(p, u + v);
        self.color
    }
}

impl Texture for Checkertexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = ((10.0 * p.0).sin()) * ((10.0 * p.1).sin()) * ((10.0 * p.2).sin());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
