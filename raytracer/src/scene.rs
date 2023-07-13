use crate::material::*;
use crate::perlin::Perlin;
use crate::shape::*;
use crate::texture::*;
use crate::tool::*;
use crate::vec3::*;
use std::sync::Arc;

const POINT_COUNT: i32 = 256;

type Object = (Arc<dyn Material>, Arc<dyn Shape>);

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

pub fn two_spheres(v: &mut Vec<Object>) {
    let texture = Checkertexture {
        odd: Arc::new(Solidcolor {
            color: Vec3(0.2, 0.3, 0.1),
        }),
        even: Arc::new(Solidcolor {
            color: Vec3(0.9, 0.9, 0.9),
        }),
    };
    let a = Lambertian {
        albebo: Arc::new(texture.clone()),
    };
    let b = Sphere {
        cent: Vec3(0.0, -10.0, 0.0),
        radi: 10.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let a = Lambertian {
        albebo: Arc::new(texture),
    };
    let b = Sphere {
        cent: Vec3(0.0, 10.0, 0.0),
        radi: 10.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
}

pub fn two_perlin_spheres(v: &mut Vec<Object>) {
    let mut perlin = Perlin {
        ranvec: [Vec3(0.0, 0.0, 0.0); POINT_COUNT as usize],
        perm_x: [0; POINT_COUNT as usize],
        perm_y: [0; POINT_COUNT as usize],
        perm_z: [0; POINT_COUNT as usize],
    };
    perlin.build();
    let texture = Noisetexture {
        noise: perlin,
        scale: 4.0,
    };
    let a = Lambertian {
        albebo: Arc::new(texture.clone()),
    };
    let b = Sphere {
        cent: Vec3(0.0, -1000.0, 0.0),
        radi: 1000.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let a = Lambertian {
        albebo: Arc::new(texture),
    };
    let b = Sphere {
        cent: Vec3(0.0, 2.0, 0.0),
        radi: 2.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
}

pub fn earth(v: &mut Vec<Object>) {
    let mut texture = Imagetexture {
        data: None,
        width: 0,
        height: 0,
    };
    texture.build("raytracer/sources/earthmap.jpg");
    let a = Lambertian {
        albebo: Arc::new(texture.clone()),
    };
    let b = Sphere {
        cent: Vec3(0.0, 0.0, 0.0),
        radi: 2.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
}

pub fn simple_light(v: &mut Vec<Object>) {
    let mut perlin = Perlin {
        ranvec: [Vec3(0.0, 0.0, 0.0); POINT_COUNT as usize],
        perm_x: [0; POINT_COUNT as usize],
        perm_y: [0; POINT_COUNT as usize],
        perm_z: [0; POINT_COUNT as usize],
    };
    perlin.build();
    let texture = Noisetexture {
        noise: perlin,
        scale: 4.0,
    };
    let a = Lambertian {
        albebo: Arc::new(texture.clone()),
    };
    let b = Sphere {
        cent: Vec3(0.0, -1000.0, 0.0),
        radi: 1000.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let a = Lambertian {
        albebo: Arc::new(texture),
    };
    let b = Sphere {
        cent: Vec3(0.0, 2.0, 0.0),
        radi: 2.0,
    };
    v.push((Arc::new(a), Arc::new(b)));

    let texture = Solidcolor {
        color: Vec3(4.0, 4.0, 4.0),
    };
    let a = Diffuselight {
        emit: Arc::new(texture),
    };
    let b = Xyrect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
}
