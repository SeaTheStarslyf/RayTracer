use crate::ray::*;
use crate::texture::*;
use crate::tool::*;
use crate::vec3::*;
use std::sync::Arc;

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct Lambertian {
    pub albebo: Arc<dyn Texture>,
    //    pub name: Name,
}

pub struct Metal {
    pub albebo: Vec3,
    pub fuzz: f64,
    //    pub name: Name,
}

pub struct Dielectric {
    pub ref_idx: f64,
}

pub struct Diffuselight {
    pub emit: Arc<dyn Texture>,
}

impl Material for Lambertian {
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
            tm: r_in.tm,
        };
        *scattered = ray;
        *attenuation = self.albebo.value(rec.u, rec.v, rec.p);
        true
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Material for Metal {
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
        let random = random_in_unit_sphere();
        let ray = Ray {
            ori: rec.p,
            dir: add(
                reflected,
                Vec3(
                    self.fuzz * random.0,
                    self.fuzz * random.1,
                    self.fuzz * random.2,
                ),
            ),
            tm: r_in.tm,
        };
        *scattered = ray;
        *attenuation = self.albebo;
        dot(scattered.dir, rec.normal) > 0.0
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3(1.0, 1.0, 1.0);
        let etai_over_etat: f64 = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let length: f64 = dot(r_in.dir, r_in.dir).sqrt();
        let unit_direction = Vec3(
            r_in.dir.0 / length,
            r_in.dir.1 / length,
            r_in.dir.2 / length,
        );
        let cos_theta = fmin(dot(multi(unit_direction, -1.0), rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, rec.normal);
            *scattered = Ray {
                ori: rec.p,
                dir: reflected,
                tm: r_in.tm,
            };
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random_double(0.0, 1.0) < reflect_prob {
            let reflected = reflect(unit_direction, rec.normal);
            *scattered = Ray {
                ori: rec.p,
                dir: reflected,
                tm: r_in.tm,
            };
            return true;
        }
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray {
            ori: rec.p,
            dir: refracted,
            tm: r_in.tm,
        };
        true
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Material for Diffuselight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &Hitrecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
