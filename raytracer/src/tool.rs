use crate::material::*;
use crate::shape::*;
use crate::texture::*;
use crate::vec3::*;
use rand::Rng;
use std::f64::consts::PI;
use std::sync::Arc;

type Object = (Arc<dyn Material>, Arc<dyn Shape>);

/*pub enum Name {
    Nlambertian,
    Nmetal,
}*/

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}
pub fn unit_vector(a: Vec3) -> Vec3 {
    let length: f64 = dot(a, a).sqrt();
    Vec3(a.0 / length, a.1 / length, a.2 / length)
}
pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}
pub fn random_vector(min: f64, max: f64) -> Vec3 {
    Vec3(
        random_double(min, max),
        random_double(min, max),
        random_double(min, max),
    )
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
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3(random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0);
        if dot(p, p) < 1.0 {
            return p;
        }
    }
}
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
pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
    let theta = (-p.1).acos();
    let phi = (-p.2).atan2(p.0) + PI;
    *u = phi / (2.0 * PI);
    *v = theta / PI;
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
pub fn multivec3(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(a.0 * b.0, a.1 * b.1, a.2 * b.2)
}
pub fn divis(a: Vec3, b: f64) -> Vec3 {
    Vec3(a.0 / b, a.1 / b, a.2 / b)
}
pub fn fmin(a: f64, b: f64) -> f64 {
    if a <= b {
        a
    } else {
        b
    }
}

pub fn random_scene(v: &mut Vec<Object>) {
    let texture = Checkertexture {
        odd: Arc::new(Solidcolor {
            color: Vec3(0.2, 0.3, 0.1),
        }),
        even: Arc::new(Solidcolor {
            color: Vec3(0.9, 0.9, 0.9),
        }),
    };
    let a = Lambertian {
        albebo: Arc::new(texture),
    };
    let b = Sphere {
        cent: Vec3(0.0, -1000.0, 0.0),
        radi: 1000.0,
    };
    v.push((Arc::new(a), Arc::new(b)));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Vec3(
                (i as f64) + 0.9 * random_double(0.0, 1.0),
                0.2,
                (j as f64) + 0.9 * random_double(0.0, 1.0),
            );
            if dot(
                reduce(center, Vec3(4.0, 0.2, 0.0)),
                reduce(center, Vec3(4.0, 0.2, 0.0)),
            )
            .sqrt()
                > 0.9
            {
                if choose_mat < 0.8 {
                    let texture = Solidcolor {
                        color: multivec3(random_vector(0.0, 1.0), random_vector(0.0, 1.0)),
                    };
                    let a = Lambertian {
                        albebo: Arc::new(texture),
                    };
                    let b = MovingSphere {
                        center0: center,
                        center1: add(center, Vec3(0.0, random_double(0.0, 0.5), 0.0)),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                    };
                    v.push((Arc::new(a), Arc::new(b)));
                } else if choose_mat < 0.95 {
                    let a = Metal {
                        albebo: random_vector(0.5, 1.0),
                        fuzz: random_double(0.0, 0.5),
                    };
                    let b = Sphere {
                        cent: center,
                        radi: 0.2,
                    };
                    v.push((Arc::new(a), Arc::new(b)));
                } else {
                    let a = Dielectric { ref_idx: 1.5 };
                    let b = Sphere {
                        cent: center,
                        radi: 0.2,
                    };
                    v.push((Arc::new(a), Arc::new(b)));
                }
            }
        }
    }

    let a = Dielectric { ref_idx: 1.5 };
    let b = Sphere {
        cent: Vec3(0.0, 1.0, 0.0),
        radi: 1.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let texture = Solidcolor {
        color: Vec3(0.4, 0.2, 0.1),
    };
    let a = Lambertian {
        albebo: Arc::new(texture),
    };
    let b = Sphere {
        cent: Vec3(-4.0, 1.0, 0.0),
        radi: 1.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let a = Metal {
        albebo: Vec3(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    let b = Sphere {
        cent: Vec3(4.0, 1.0, 0.0),
        radi: 1.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
}
