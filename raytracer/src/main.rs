mod camera;
mod material;
mod perlin;
mod ray;
mod scene;
mod shape;
mod texture;
mod tool;
mod vec3;
use crate::camera::*;
use crate::material::*;
use crate::ray::*;
use crate::scene::*;
use crate::shape::*;
use crate::tool::*;
use crate::vec3::*;
//use crate::texture::*;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
//use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs::File, process::exit};

type Object = (Arc<dyn Material>, Arc<dyn Shape>);

fn hit_shape(v: &[Object], r: Ray) -> Hitrecord {
    let mut ans: f64 = 0x10000000 as f64;
    let mut hitrecord = Hitrecord {
        p: Vec3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0),
        t: -1.0,
        u: 0.0,
        v: 0.0,
        front_face: false,
        num: -1,
    };
    for (js, i) in (0_i32..).zip(v.iter()) {
        if i.1.gethit(r, &mut hitrecord, 0.001, ans, js) {
            ans = hitrecord.t;
        }
    }
    hitrecord
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
            tm: 0.0,
        }; //scatter就是nexray
        let object = &v[hit.num as usize];
        let mut attenuation = Vec3(0.0, 0.0, 0.0);
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
    let path = std::path::Path::new("output/book2/image13.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;
    //    let width = 400;
    //    let height = 200;
    let quality = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;
    //    let img: RgbImage = ImageBuffer::new(width, height);
    let mut v: Vec<(Arc<dyn Material>, Arc<dyn Shape>)> = Vec::new();

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    //Add Object and Camera
    let mut cam = Camera {
        origin: Vec3(0.0, 0.0, 0.0),
        lower_left_corner: Vec3(0.0, 0.0, 0.0),
        horizontal: Vec3(0.0, 0.0, 0.0),
        vertical: Vec3(0.0, 0.0, 0.0),
        u: Vec3(0.0, 0.0, 0.0),
        v: Vec3(0.0, 0.0, 0.0),
        w: Vec3(0.0, 0.0, 0.0),
        lens_radius: 0.0,
        time0: 0.0,
        time1: 0.0,
    };
    let number = 3;
    match number {
        1 => {
            random_scene(&mut v);
            let lookfrom1 = Vec3(13.0, 2.0, 3.0);
            let lookat1 = Vec3(0.0, 0.0, 0.0);
            let para = Camerapara {
                lookfrom: lookfrom1,
                lookat: lookat1,
                vup: Vec3(0.0, 1.0, 0.0),
                vfov: 20.0,
                aspect: aspect_ratio,
                aperture: 0.1, //光圈直径
                focus_dist: 10.0,
                t0: 0.0,
                t1: 1.0,
            };
            cam.build(para);
        }
        2 => {
            two_spheres(&mut v);
            let lookfrom1 = Vec3(13.0, 2.0, 3.0);
            let lookat1 = Vec3(0.0, 0.0, 0.0);
            let para = Camerapara {
                lookfrom: lookfrom1,
                lookat: lookat1,
                vup: Vec3(0.0, 1.0, 0.0),
                vfov: 20.0,
                aspect: aspect_ratio,
                aperture: 0.0, //光圈直径
                focus_dist: 10.0,
                t0: 0.0,
                t1: 1.0,
            };
            cam.build(para);
        }
        3 => {
            two_perlin_spheres(&mut v);
            let lookfrom1 = Vec3(13.0, 2.0, 3.0);
            let lookat1 = Vec3(0.0, 0.0, 0.0);
            let para = Camerapara {
                lookfrom: lookfrom1,
                lookat: lookat1,
                vup: Vec3(0.0, 1.0, 0.0),
                vfov: 20.0,
                aspect: aspect_ratio,
                aperture: 0.0, //光圈直径
                focus_dist: 10.0,
                t0: 0.0,
                t1: 1.0,
            };
            cam.build(para);
        }
        _ => {
            println!("Have not set such a scene!");
        }
    }

    //Render
    let shared_v: Arc<Mutex<Vec<Object>>> = Arc::new(Mutex::new(v));
    let img: Arc<Mutex<RgbImage>> = Arc::new(Mutex::new(ImageBuffer::new(width, height)));
    let threads = 6; // 获取可用CPU核心数
    let rows_per_thread = height as f64 / threads as f64;
    //    let threads = num_cpus::get();

    let handles: Vec<_> = (0..threads)
        .map(|tid| {
            let img = Arc::clone(&img);
            let shared_v = Arc::clone(&shared_v);
            let cam = cam;
            let progress = progress.clone();

            thread::spawn(move || {
                let start_row = (tid as f64 * rows_per_thread) as u32;
                let end_row = ((tid + 1) as f64 * rows_per_thread) as u32;
                let mut locked_img = img.lock().unwrap();
                let v = shared_v.lock().unwrap();
                for j in (start_row..end_row).rev() {
                    for i in 0..width {
                        let pixel = locked_img.get_pixel_mut(i, height - 1 - j);
                        let mut colorend = Vec3(0.0, 0.0, 0.0);
                        for _s in 0..samples_per_pixel {
                            let u: f64 = (i as f64 + random_double(0.0, 1.0)) / (width as f64);
                            let w: f64 = (j as f64 + random_double(0.0, 1.0)) / (height as f64);
                            let r = cam.get_ray(u, w);
                            let color = ray_color(r, &v, max_depth);
                            colorend = add(colorend, color);
                        }
                        let r: f64 = (colorend.0 / (samples_per_pixel as f64)).sqrt() * 255.999;
                        let g: f64 = (colorend.1 / (samples_per_pixel as f64)).sqrt() * 255.999;
                        let b: f64 = (colorend.2 / (samples_per_pixel as f64)).sqrt() * 255.999;
                        *pixel = image::Rgb([r as u8, g as u8, b as u8]);
                    }
                    progress.inc(threads as u64);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img.lock().unwrap().clone());
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
