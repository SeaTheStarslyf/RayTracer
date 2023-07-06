use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

struct Vec3(f64, f64, f64);

struct Ray {
    ori: Vec3,
    dir: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        Vec3(
            self.ori.0 + self.dir.0 * t,
            self.ori.1 + self.dir.1 * t,
            self.ori.2 + self.dir.2 * t,
        )
    }
    fn ray_color(&self) -> Vec3 {
        let length: f64 =
            (self.dir.0 * self.dir.0 + self.dir.1 * self.dir.1 + self.dir.2 * self.dir.2).sqrt();
        let t: f64 = 0.5 * (-self.dir.1 / length + 1.0);
        Vec3(
            (1.0 - t) * 1.0 + t * 0.5,
            (1.0 - t) * 1.0 + t * 0.7,
            (1.0 - t) * 1.0 + t * 1.0,
        )
    }
}

fn main() {
    let path = std::path::Path::new("output/book1/image2.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let width = 200;
    let height = 100;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let origin = Vec3(0.0, 0.0, 0.0);
    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);
            let u = (i as f64) / (width as f64);
            let v: f64 = (j as f64) / (height as f64);
            let dir = Vec3(-2.0 + u * 4.0, -1.0 + v * 2.0, -1.0);
            let r = Ray {
                ori: Vec3(origin.0, origin.1, origin.2),
                dir: dir,
            };
            let color = r.ray_color();
            let r: f64 = color.0 * 255.999;
            let g: f64 = color.1 * 255.999;
            let b: f64 = color.2 * 255.999;
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
            //            Writecolor(color, i, j);
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
