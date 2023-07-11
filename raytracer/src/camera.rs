use crate::ray::*;
use crate::tool::*;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn build(&mut self, vfov: f64, aspect: f64) {
        self.origin = Vec3(0.0, 0.0, 0.0);
        let theta = degrees_to_radians(vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        self.lower_left_corner = Vec3(-half_width, -half_height, -1.0);
        self.horizontal = Vec3(2.0 * half_width, 0.0, 0.0);
        self.vertical = Vec3(0.0, 2.0 * half_height, 0.0);
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            ori: self.origin,
            dir: reduce(
                add(
                    add(self.lower_left_corner, multi(self.horizontal, u)),
                    multi(self.vertical, v),
                ),
                self.origin,
            ),
        }
    }
}
