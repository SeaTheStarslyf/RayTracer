use crate::ray::*;
use crate::tool::*;
use crate::vec3::*;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn getcent(&self) -> Vec3;
    fn getradi(&self) -> f64;
    fn getalbebo(&self) -> Vec3;
}

pub struct LambertianBall {
    pub cent: Vec3,
    pub radi: f64,
    pub albebo: Vec3,
    //    pub name: Name,
}

impl Material for LambertianBall {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let _none: Vec3 = r_in.ori;

        let scatter_direction: Vec3 = add(rec.normal, random_unit_vector());
        let ray = Ray {
            ori: rec.p,
            dir: scatter_direction,
        };
        *scattered = ray;
        *attenuation = self.albebo;
        true
    }
    fn getcent(&self) -> Vec3 {
        self.cent
    }
    fn getradi(&self) -> f64 {
        self.radi
    }
    fn getalbebo(&self) -> Vec3 {
        self.albebo
    }
}

pub struct MetalBall {
    pub cent: Vec3,
    pub radi: f64,
    pub albebo: Vec3,
    //    pub name: Name,
}

impl Material for MetalBall {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let length: f64 = dot(r_in.dir, r_in.dir).sqrt();
        let unit = Vec3(
            r_in.dir.0 / length,
            r_in.dir.1 / length,
            r_in.dir.2 / length,
        );
        let reflected: Vec3 = reflect(unit, rec.normal);
        let ray = Ray {
            ori: rec.p,
            dir: reflected,
        };
        *scattered = ray;
        *attenuation = self.albebo;
        dot(scattered.dir, rec.normal) > 0.0
    }
    fn getcent(&self) -> Vec3 {
        self.cent
    }
    fn getradi(&self) -> f64 {
        self.radi
    }
    fn getalbebo(&self) -> Vec3 {
        self.albebo
    }
}
