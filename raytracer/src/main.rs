mod material;
mod ray;
mod tool;
mod vec3;
use crate::material::*;
use crate::ray::*;
use crate::tool::*;
use crate::vec3::*;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};
//const PI: f64 = 3.1415926535897932385;

fn hit_sphere(v: &[Box<dyn Material>], r: Ray) -> Hitrecord {
    let mut ans: f64 = 0x10000000 as f64;
    let mut n: i32 = -1;
    for (js, i) in (0_i32..).zip(v.iter()) {
        let center: Vec3 = i.getcent();
        let radius: f64 = i.getradi();
        let oc = Vec3(r.ori.0 - center.0, r.ori.1 - center.1, r.ori.2 - center.2);
        let a: f64 = dot(r.dir, r.dir);
        let b: f64 = 2.0 * dot(oc, r.dir);
        let c: f64 = dot(oc, oc) - radius * radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t: f64 = (-b - discriminant.sqrt()) / (2.0 * a); //注意取近时容易出错
            if t < ans && t > 0.001 {
                ans = t;
                n = js;
                continue;
            }
            let t: f64 = (-b + discriminant.sqrt()) / (2.0 * a);
            if t < ans && t > 0.001 {
                ans = t;
                n = js;
            }
        }
    }
    if n == -1 {
        Hitrecord {
            p: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: -1.0,
            num: -1,
        }
    } else {
        let point = r.at(ans);
        let object = &v[n as usize];
        let ray = reduce(point, object.getcent());
        let length: f64 = dot(ray, ray).sqrt();
        let nor = Vec3(ray.0 / length, ray.1 / length, ray.2 / length);
        Hitrecord {
            p: point,
            normal: nor,
            t: ans,
            num: n,
        } //p实际上是交点或者说终点的坐标,在00原点系下可以正确表示一些东西
    }
}

fn ray_color(r: Ray, v: &Vec<Box<dyn Material>>, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }
    let hit: Hitrecord = hit_sphere(v, r);
    if hit.t > 0.0 {
        let mut scattered = Ray {
            ori: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(0.0, 0.0, 0.0),
        }; //scatter就是nexray
        let object = &v[hit.num as usize];
        let mut attenuation = object.getalbebo();
        if object.scatter(&r, &hit, &mut attenuation, &mut scattered) {
            let nex = ray_color(scattered, v, depth - 1);
            return Vec3(
                attenuation.0 * nex.0,
                attenuation.1 * nex.1,
                attenuation.2 * nex.2,
            );
        }
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
    let path = std::path::Path::new("output/book1/image11.jpg");
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
    let mut v: Vec<Box<dyn Material>> = Vec::new();

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let origin = Vec3(0.0, 0.0, 0.0);
    let b = LambertianBall {
        cent: Vec3(0.0, 0.0, -1.0),
        radi: 0.5,
        albebo: Vec3(0.7, 0.3, 0.3),
    };
    v.push(Box::new(b));
    let b = LambertianBall {
        cent: Vec3(0.0, -100.5, -1.0),
        radi: 100.0,
        albebo: Vec3(0.8, 0.8, 0.0),
    };
    v.push(Box::new(b));
    let b = MetalBall {
        cent: Vec3(1.0, 0.0, -1.0),
        radi: 0.5,
        albebo: Vec3(0.8, 0.6, 0.2),
    };
    v.push(Box::new(b));
    let b = MetalBall {
        cent: Vec3(-1.0, 0.0, -1.0),
        radi: 0.5,
        albebo: Vec3(0.8, 0.8, 0.8),
    };
    v.push(Box::new(b));

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
