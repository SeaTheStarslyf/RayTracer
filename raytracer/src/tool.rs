use crate::vec3::*;
use rand::Rng;

/*pub enum Name {
    Nlambertian,
    Nmetal,
}*/

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}
pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3(
            random_double(-1.0, 1.0),
            random_double(-1.0, 1.0),
            random_double(-1.0, 1.0),
        );
        if dot(p, p).sqrt() < 1.0 {
            return p;
        }
    }
}
pub fn random_unit_vector() -> Vec3 {
    let p = random_in_unit_sphere();
    let length = dot(p, p).sqrt();
    Vec3(p.0 / length, p.1 / length, p.2 / length)
}
/*pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_init_sphere = random_in_unit_sphere();
    if dot(in_init_sphere, normal) > 0.0 {
        in_init_sphere
    } else {
        Vec3(-in_init_sphere.0, -in_init_sphere.1, -in_init_sphere.2)
    }
}*/
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    let length: f64 = dot(v, n);
    Vec3(
        v.0 - 2.0 * length * n.0,
        v.1 - 2.0 * length * n.1,
        v.2 - 2.0 * length * n.2,
    )
}
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let uv_rfl = Vec3(-uv.0, -uv.1, -uv.2);
    let cos_theta: f64 = dot(uv_rfl, n);
    let r_out_parallel = multi(add(uv, multi(n, cos_theta)), etai_over_etat);
    let r_out_perp = multi(n, -(1.0 - dot(r_out_parallel, r_out_parallel)).sqrt());
    add(r_out_parallel, r_out_perp)
}
pub fn add(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(a.0 + b.0, a.1 + b.1, a.2 + b.2)
}
pub fn reduce(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(a.0 - b.0, a.1 - b.1, a.2 - b.2)
}
pub fn multi(a: Vec3, b: f64) -> Vec3 {
    Vec3(a.0 * b, a.1 * b, a.2 * b)
}
pub fn fmin(a: f64, b: f64) -> f64 {
    if a <= b {
        a
    } else {
        b
    }
}
