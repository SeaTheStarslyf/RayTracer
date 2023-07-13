//use crate::ray::*;
use crate::perlin::*;
use crate::tool::*;
use crate::vec3::*;
use std::sync::Arc;
//use std::sync::{Arc, Mutex};
//use std::path::Path;
//use stb_image::image::LoadResult;
//use std::os::raw::c_char;
//use image::{self, GenericImageView, DynamicImage, ImageBuffer};
use image::{self, DynamicImage, ImageBuffer};

//const BYTES_PER_PIXEL: i32 = 3;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
    fn build(&mut self, filename: &str);
}

#[derive(Clone)]
pub struct Solidcolor {
    pub color: Vec3,
}

#[derive(Clone)]
pub struct Checkertexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

#[derive(Clone)]
pub struct Noisetexture {
    pub noise: Perlin,
    pub scale: f64,
}

#[derive(Clone)]
pub struct Imagetexture {
    pub data: Option<ImageBuffer<image::Rgb<u8>, Vec<u8>>>,
    pub width: u32,
    pub height: u32,
}

impl Texture for Solidcolor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let _haha = multi(p, u + v);
        self.color
    }
    fn build(&mut self, _filename: &str) {}
}

impl Texture for Checkertexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = ((10.0 * p.0).sin()) * ((10.0 * p.1).sin()) * ((10.0 * p.2).sin());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
    fn build(&mut self, _filename: &str) {}
}

impl Texture for Noisetexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let _haha = u + v;
        multi(
            multi(Vec3(1.0, 1.0, 1.0), 0.5),
            1.0 + (self.scale * p.2 + 10.0 * self.noise.turb(p, 7)).sin(),
        )
    }
    fn build(&mut self, _filename: &str) {}
}

impl Texture for Imagetexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        if self.data.is_none() {
            return Vec3(0.0, 1.0, 1.0);
        }

        let u_clamped = u.clamp(0.0, 1.0);
        let v_clamped = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u_clamped * self.width as f64) as u32;
        let mut j = (v_clamped * self.height as f64) as u32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        const COLOR_SCALE: f64 = 1.0 / 255.0;
        if let Some(data) = &self.data {
            let pixel = data.get_pixel(i, j);
            Vec3(
                COLOR_SCALE * pixel[0] as f64,
                COLOR_SCALE * pixel[1] as f64,
                COLOR_SCALE * pixel[2] as f64,
            )
        } else {
            Vec3(0.0, 1.0, 1.0)
        }
    }
    fn build(&mut self, filename: &str) {
        let img_result: image::ImageResult<DynamicImage> = image::open(filename);

        if let Ok(img) = img_result {
            let rgb_img = img.to_rgb8();
            let width = rgb_img.width();
            let height = rgb_img.height();

            self.data = Some(rgb_img);
            self.width = width;
            self.height = height;
        } else {
            eprintln!("ERROR: Could not load texture image file '{}'.", filename);

            self.data = None;
            self.width = 0;
            self.height = 0;
        }
    }
}
