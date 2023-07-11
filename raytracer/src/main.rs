mod material;
mod ray;
mod shape;
mod tool;
mod vec3;
use crate::material::*;
use crate::ray::*;
use crate::shape::*;
use crate::tool::*;
use crate::vec3::*;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};
//const PI: f64 = 3.1415926535897932385;
type Object = (Box<dyn Material>, Box<dyn Shape>);

fn hit_shape(v: &[Object], r: Ray) -> Hitrecord {
    let mut ans: f64 = 0x10000000 as f64;
    let mut n: i32 = -1;
    for (js, i) in (0_i32..).zip(v.iter()) {
        let t1 = i.1.gethit(r);
        if t1 > 0.001 && t1 < ans {
            ans = t1;
            n = js;
        }
    }
    if n == -1 {
        Hitrecord {
            p: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: -1.0,
            num: -1,
            front_face: false,
        }
    } else {
        let point = r.at(ans);
        let object = &v[n as usize];
        let ray = reduce(point, object.1.getcent());
        let length: f64 = dot(ray, ray).sqrt();
        let nor = Vec3(ray.0 / length, ray.1 / length, ray.2 / length);
        let mut hitrecord = Hitrecord {
            p: point,
            normal: nor,
            t: ans,
            num: n,
            front_face: false,
        }; //p实际上是交点或者说终点的坐标,在00原点系下可以正确表示一些东西
        hitrecord.set_face_normal(r, nor);
        hitrecord
    }
}

fn ray_color(r: Ray, v: &Vec<Object>, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }
    let hit: Hitrecord = hit_shape(v, r);
    if hit.t > 0.0 {
        let mut scattered = Ray {
            ori: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(0.0, 0.0, 0.0),
        }; //scatter就是nexray
        let object = &v[hit.num as usize];
        let mut attenuation = object.0.getalbebo();
        if object.0.scatter(&r, &hit, &mut attenuation, &mut scattered) {
            let nex = ray_color(scattered, v, depth - 1);
            return Vec3(
                attenuation.0 * nex.0,
                attenuation.1 * nex.1,
                attenuation.2 * nex.2,
            );
        }
        return Vec3(0.0, 0.0, 0.0);
        //            return Vec3(0.5 * (n.0 + 1.0), 0.5 * (n.1 + 1.0), 0.5 * (n.2 + 1.0));
    }
    let length: f64 = dot(r.dir, r.dir).sqrt();
    let t: f64 = 0.5 * (r.dir.1 / length + 1.0);
    Vec3(
        (1.0 - t) * 1.0 + t * 0.5,
        (1.0 - t) * 1.0 + t * 0.7,
        (1.0 - t) * 1.0 + t * 1.0,
    )
}

fn main() {
    let path = std::path::Path::new("output/book1/image13.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //    let aspect_ratio = 16.0 / 9.0;
    //    let width = 400;
    //    let height = (width as f64 / aspect_ratio) as u32;
    let width = 400;
    let height = 200;
    let quality = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let mut v: Vec<(Box<dyn Material>, Box<dyn Shape>)> = Vec::new();

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let origin = Vec3(0.0, 0.0, 0.0);
    let a = Dielectric { ref_idx: 1.5 };
    let b = Sphere {
        cent: Vec3(0.0, 0.0, -1.0),
        radi: 0.5,
    };
    v.push((Box::new(a), Box::new(b)));
    let a = Lambertian {
        albebo: Vec3(0.8, 0.8, 0.0),
    };
    let b = Sphere {
        cent: Vec3(0.0, -100.5, -1.0),
        radi: 100.0,
    };
    v.push((Box::new(a), Box::new(b)));
    let a = Metal {
        albebo: Vec3(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };
    let b = Sphere {
        cent: Vec3(1.0, 0.0, -1.0),
        radi: 0.5,
    };
    v.push((Box::new(a), Box::new(b)));
    let a = Dielectric { ref_idx: 1.5 };
    let b = Sphere {
        cent: Vec3(-1.0, 0.0, -1.0),
        radi: 0.5,
    };
    v.push((Box::new(a), Box::new(b)));

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, height - 1 - j);
            let mut colorend = Vec3(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u: f64 = (i as f64 + random_double(0.0, 1.0)) / (width as f64);
                let w: f64 = (j as f64 + random_double(0.0, 1.0)) / (height as f64);
                let direction = Vec3(-2.0 + u * 4.0, -1.0 + w * 2.0, -1.0);
                let r = Ray {
                    ori: Vec3(origin.0, origin.1, origin.2),
                    dir: direction,
                };
                let color = ray_color(r, &v, max_depth);
                colorend = add(colorend, color);
            }
            let r: f64 = (colorend.0 / (samples_per_pixel as f64)).sqrt() * 255.999;
            let g: f64 = (colorend.1 / (samples_per_pixel as f64)).sqrt() * 255.999;
            let b: f64 = (colorend.2 / (samples_per_pixel as f64)).sqrt() * 255.999;
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
