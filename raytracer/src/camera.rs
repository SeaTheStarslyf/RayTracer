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
    pub fn build(&mut self, lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) {
        self.origin = lookfrom;
        let theta = degrees_to_radians(vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(reduce(lookfrom, lookat));
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        self.lower_left_corner = reduce(
            reduce(
                reduce(self.origin, multi(u, half_width)),
                multi(v, half_height),
            ),
            w,
        );
        self.horizontal = multi(u, 2.0 * half_width);
        self.vertical = multi(v, 2.0 * half_height);
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
