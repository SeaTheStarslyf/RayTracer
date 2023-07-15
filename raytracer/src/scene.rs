use crate::getobj::getobject;
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

pub fn cornell_box(v: &mut Vec<Object>) {
    let light = Solidcolor {
        color: Vec3(7.0, 7.0, 7.0),
    };
    let red = Solidcolor {
        color: Vec3(0.65, 0.05, 0.05),
    };
    let white = Solidcolor {
        color: Vec3(0.73, 0.73, 0.73),
    };
    let green = Solidcolor {
        color: Vec3(0.12, 0.45, 0.15),
    };

    let a = Lambertian {
        albebo: Arc::new(green),
    };
    let b = Yzrect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let a = Lambertian {
        albebo: Arc::new(red),
    };
    let b = Yzrect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let a = Diffuselight {
        emit: Arc::new(light),
    };
    let b = Xzrect {
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let a = Lambertian {
        albebo: Arc::new(white.clone()),
    };
    let b = Xzrect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let a = Lambertian {
        albebo: Arc::new(white.clone()),
    };
    let b = Xzrect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    v.push((Arc::new(a), Arc::new(b)));
    let a = Lambertian {
        albebo: Arc::new(white.clone()),
    };
    let b = Xyrect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
    };
    v.push((Arc::new(a), Arc::new(b.clone())));

    /*    let a = Lambertian {
        albebo: Arc::new(white.clone()),
    };*/
    let randians = degrees_to_radians(15.0);
    let sin_theta1 = randians.sin();
    let cos_theta1 = randians.cos();
    let mut rotatey = Rotatey {
        ptr: Arc::new(b.clone()),
        sin_theta: sin_theta1,
        cos_theta: cos_theta1,
    };
    let mut translate1 = Translate {
        ptr: Arc::new(b.clone()),
        offset: Vec3(265.0, 0.0, 295.0),
    };
    let mut box1 = Box {
        box_max: Vec3(0.0, 0.0, 0.0),
        box_min: Vec3(0.0, 0.0, 0.0),
        sides: Vec::new(),
        rotat: Arc::new(rotatey.clone()),
        trans: Arc::new(translate1.clone()),
    };
    box1.buildbox(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 330.0, 165.0),
        Arc::new(white.clone()),
    );
    rotatey.ptr = Arc::new(box1.clone());
    translate1.ptr = Arc::new(rotatey);
    //    v.push((Arc::new(a), Arc::new(translate)));
    /*    let a = Lambertian {
        albebo: Arc::new(white.clone()),
    };*/
    let randians = degrees_to_radians(-18.0);
    let sin_theta1 = randians.sin();
    let cos_theta1 = randians.cos();
    let mut rotatey = Rotatey {
        ptr: Arc::new(b.clone()),
        sin_theta: sin_theta1,
        cos_theta: cos_theta1,
    };
    let mut translate2 = Translate {
        ptr: Arc::new(b),
        offset: Vec3(130.0, 0.0, 65.0),
    };
    let mut box2 = Box {
        box_max: Vec3(0.0, 0.0, 0.0),
        box_min: Vec3(0.0, 0.0, 0.0),
        sides: Vec::new(),
        rotat: Arc::new(rotatey.clone()),
        trans: Arc::new(translate2.clone()),
    };
    box2.buildbox(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 165.0, 165.0),
        Arc::new(white),
    );
    rotatey.ptr = Arc::new(box2.clone());
    translate2.ptr = Arc::new(rotatey);
    //    v.push((Arc::new(a), Arc::new(translate)));

    let col = Solidcolor {
        color: Vec3(0.0, 0.0, 0.0),
    };
    /*    let a = Lambertian {
        albebo: Arc::new(col.clone()),
    };*/
    let f = Isotropic {
        albebo: Arc::new(col),
    };
    let frg = Constantmedium {
        boundary: Arc::new(translate1),
        phase_function: Arc::new(f.clone()),
        neg_inv_density: -1.0 / 0.01,
    };
    v.push((Arc::new(f), Arc::new(frg)));

    let col = Solidcolor {
        color: Vec3(1.0, 1.0, 1.0),
    };
    /*     let a = Lambertian {
        albebo: Arc::new(col.clone()),
    };*/
    let f = Isotropic {
        albebo: Arc::new(col),
    };
    let frg = Constantmedium {
        boundary: Arc::new(translate2),
        phase_function: Arc::new(f.clone()),
        neg_inv_density: -1.0 / 0.01,
    };
    v.push((Arc::new(f), Arc::new(frg)));
}

pub fn final_scene(v: &mut Vec<Object>) {
/*    let groundcolor = Solidcolor {
        color: Vec3(0.48, 0.83, 0.53),
    };
    let ground = Lambertian {
        albebo: Arc::new(groundcolor.clone()),
    };

    let b = Sphere {
        cent: Vec3(0.0, 2.0, 0.0),
        radi: 2.0,
    };

    let boxed_per_side = 20;
    for i in 0..boxed_per_side {
        for j in 0..boxed_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;

            let randians = degrees_to_radians(0.0);
            let sin_theta1 = randians.sin();
            let cos_theta1 = randians.cos();
            let rotatey = Rotatey {
                ptr: Arc::new(b.clone()),
                sin_theta: sin_theta1,
                cos_theta: cos_theta1,
            };
            let translate1 = Translate {
                ptr: Arc::new(b.clone()),
                offset: Vec3(0.0, 0.0, 0.0),
            };
            let mut box1 = Box {
                box_max: Vec3(0.0, 0.0, 0.0),
                box_min: Vec3(0.0, 0.0, 0.0),
                sides: Vec::new(),
                rotat: Arc::new(rotatey.clone()),
                trans: Arc::new(translate1.clone()),
            };
            box1.buildbox(
                Vec3(x0, y0, z0),
                Vec3(x1, y1, z1),
                Arc::new(groundcolor.clone()),
            );
            v.push((Arc::new(ground.clone()), Arc::new(box1)));
        }
    }*/
/*    let texture = Solidcolor {
        color: Vec3(0.48, 0.83, 0.53),
    };
    let a = Lambertian {
        albebo: Arc::new(texture),
    };
    let b = Sphere {
        cent: Vec3(0.0, -1000.0, 0.0),
        radi: 1000.0,
    };
    v.push((Arc::new(a), Arc::new(b)));*/

    let light = Solidcolor {
        color: Vec3(10.0, 10.0, 10.0),
    };
    let a = Diffuselight {
        emit: Arc::new(light),
    };
    let b = Xzrect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
    };
    v.push((Arc::new(a), Arc::new(b)));

    let center11 = Vec3(400.0, 400.0, 200.0);
    let center22 = add(center11, Vec3(30.0, 0.0, 0.0));

    let texture = Solidcolor {
        color: Vec3(0.7, 0.3, 0.1),
    };
    let a = Lambertian {
        albebo: Arc::new(texture),
    };
    let b = MovingSphere {
        center0: center11,
        center1: center22,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
    };
    v.push((Arc::new(a), Arc::new(b)));

    let a = Dielectric { ref_idx: 1.5 };
    let b = Sphere {
        cent: Vec3(260.0, 150.0, 45.0),
        radi: 50.0,
    };
    v.push((Arc::new(a), Arc::new(b)));

    let a = Metal {
        albebo: Vec3(0.8, 0.8, 0.9),
        fuzz: 1.0,
    };
    let b = Sphere {
        cent: Vec3(0.0, 150.0, 145.0),
        radi: 50.0,
    };
    v.push((Arc::new(a), Arc::new(b)));

    let a = Dielectric { ref_idx: 1.5 };
    let b = Sphere {
        cent: Vec3(360.0, 150.0, 145.0),
        radi: 70.0,
    };
    v.push((Arc::new(a), Arc::new(b.clone())));

    let col = Solidcolor {
        color: Vec3(0.2, 0.4, 0.9),
    };
    let f = Isotropic {
        albebo: Arc::new(col),
    };
    let frg = Constantmedium {
        boundary: Arc::new(b),
        phase_function: Arc::new(f.clone()),
        neg_inv_density: -1.0 / 0.2,
    };
    v.push((Arc::new(f), Arc::new(frg)));

    let a = Dielectric { ref_idx: 1.5 };
    let b = Sphere {
        cent: Vec3(0.0, 0.0, 0.0),
        radi: 5000.0,
    };
    let col = Solidcolor {
        color: Vec3(1.0, 1.0, 1.0),
    };
    let f = Isotropic {
        albebo: Arc::new(col),
    };
    let frg = Constantmedium {
        boundary: Arc::new(b),
        phase_function: Arc::new(f),
        neg_inv_density: -1.0 / 0.0001,
    };
    v.push((Arc::new(a), Arc::new(frg)));

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
        cent: Vec3(400.0, 200.0, 400.0),
        radi: 100.0,
    };
    v.push((Arc::new(a), Arc::new(b)));

    let mut perlin = Perlin {
        ranvec: [Vec3(0.0, 0.0, 0.0); POINT_COUNT as usize],
        perm_x: [0; POINT_COUNT as usize],
        perm_y: [0; POINT_COUNT as usize],
        perm_z: [0; POINT_COUNT as usize],
    };
    perlin.build();
    let texture = Noisetexture {
        noise: perlin,
        scale: 0.1,
    };
    let a = Lambertian {
        albebo: Arc::new(texture),
    };
    let b = Sphere {
        cent: Vec3(220.0, 280.0, 300.0),
        radi: 80.0,
    };
    v.push((Arc::new(a), Arc::new(b)));

    let whitecolor = Solidcolor {
        color: Vec3(0.73, 0.73, 0.73),
    };
    let white = Lambertian {
        albebo: Arc::new(whitecolor),
    };
    let ns = 0;
    for _j in 0..ns {
        let b = Sphere {
            cent: random_vector(0.0, 165.0),
            radi: 10.0,
        };

        let randians = degrees_to_radians(15.0);
        let sin_theta1 = randians.sin();
        let cos_theta1 = randians.cos();
        let mut rotatey = Rotatey {
            ptr: Arc::new(b.clone()),
            sin_theta: sin_theta1,
            cos_theta: cos_theta1,
        };
        let mut translate1 = Translate {
            ptr: Arc::new(b.clone()),
            offset: Vec3(-100.0, 270.0, 395.0),
        };
        rotatey.ptr = Arc::new(b.clone());
        translate1.ptr = Arc::new(rotatey);
        v.push((Arc::new(white.clone()), Arc::new(translate1)));
    }

    getobject(v, "raytracer/sources/Air_Balloon.obj");
}
