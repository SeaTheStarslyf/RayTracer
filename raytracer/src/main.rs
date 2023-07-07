use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use std::{fs::File, process::exit};

#[derive(Copy, Clone)]
struct Vec3(f64, f64, f64);

#[derive(Copy, Clone)]
struct Ray {
    ori: Vec3,
    dir: Vec3,
}

#[derive(Copy, Clone)]
struct Ball {
    cent: Vec3,
    radi: f64,
}

#[derive(Copy, Clone)]
struct Which {
    val: f64,
    num: i32,
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}
fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}
fn add(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn hit_sphere(v: Vec<Ball>, r: Ray) -> Which {
    let mut ans: f64 = 0x10000000 as f64;
    let mut n: i32 = -1;
    for (js, i) in (0_i32..).zip(v.iter()) {
        let center: Vec3 = i.cent;
        let radius: f64 = i.radi;
        let oc = Vec3(r.ori.0 - center.0, r.ori.1 - center.1, r.ori.2 - center.2);
        let a: f64 = dot(r.dir, r.dir);
        let b: f64 = 2.0 * dot(oc, r.dir);
        let c: f64 = dot(oc, oc) - radius * radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t: f64 = (-b - discriminant.sqrt()) / (2.0 * a); //注意取近时容易出错
            if t < ans && t > 0.0 {
                ans = t;
                n = js;
            }
        }
    }
    if n == -1 {
        Which { val: -1.0, num: 0 }
    } else {
        Which { val: ans, num: n }
    }
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        Vec3(
            self.ori.0 + self.dir.0 * t,
            self.ori.1 + self.dir.1 * t,
            self.ori.2 + self.dir.2 * t,
        )
    }
    fn ray_color(&self, v: Vec<Ball>) -> Vec3 {
        let r = Ray {
            ori: self.ori,
            dir: self.dir,
        };
        let ans: Which = hit_sphere(v.clone(), r);
        let t: f64 = ans.val;
        let class: i32 = ans.num;
        let ball: &Ball = &v[class as usize];
        if t > 0.0 {
            let mut ray = r.at(t);
            ray.0 -= ball.cent.0;
            ray.1 -= ball.cent.1;
            ray.2 -= ball.cent.2;
            let length: f64 = dot(ray, ray).sqrt();
            let n = Vec3(ray.0 / length, ray.1 / length, ray.2 / length);
            return Vec3(0.5 * (n.0 + 1.0), 0.5 * (n.1 + 1.0), 0.5 * (n.2 + 1.0));
        }
        let length: f64 =
            (self.dir.0 * self.dir.0 + self.dir.1 * self.dir.1 + self.dir.2 * self.dir.2).sqrt();
        let t: f64 = 0.5 * (self.dir.1 / length + 1.0);
        Vec3(
            (1.0 - t) * 1.0 + t * 0.5,
            (1.0 - t) * 1.0 + t * 0.7,
            (1.0 - t) * 1.0 + t * 1.0,
        )
    }
}

fn main() {
    let path = std::path::Path::new("output/book1/image6.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let width = 200;
    let height = 100;
    let quality = 100;
    let samples_per_pixel = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let mut v: Vec<Ball> = Vec::new();

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let origin = Vec3(0.0, 0.0, 0.0);
    let mut b = Ball {
        cent: Vec3(0.0, 0.0, -1.0),
        radi: 0.5,
    };
    v.push(b);
    b.cent = Vec3(0.0, -100.5, -1.0);
    b.radi = 100.0;
    v.push(b);

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
                let color = r.ray_color(v.clone());
                colorend = add(colorend, color);
            }
            let r: f64 = colorend.0 / (samples_per_pixel as f64) * 255.999;
            let g: f64 = colorend.1 / (samples_per_pixel as f64) * 255.999;
            let b: f64 = colorend.2 / (samples_per_pixel as f64) * 255.999;
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
