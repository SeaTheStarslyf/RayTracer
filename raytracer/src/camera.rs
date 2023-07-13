use crate::ray::*;
use crate::tool::*;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

#[derive(Copy, Clone)]
pub struct Camerapara {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aspect: f64,
    pub aperture: f64,
    pub focus_dist: f64,
    pub t0: f64,
    pub t1: f64,
}

impl Camera {
    pub fn build(&mut self, camera: Camerapara) {
        self.origin = camera.lookfrom;
        self.lens_radius = camera.aperture / 2.0;
        self.time0 = camera.t0;
        self.time1 = camera.t1;

        let theta = degrees_to_radians(camera.vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = camera.aspect * half_height;

        self.w = unit_vector(reduce(camera.lookfrom, camera.lookat));
        self.u = unit_vector(cross(camera.vup, self.w));
        self.v = cross(self.w, self.u);
        self.lower_left_corner = reduce(
            reduce(
                reduce(self.origin, multi(self.u, half_width * camera.focus_dist)),
                multi(self.v, half_height * camera.focus_dist),
            ),
            multi(self.w, camera.focus_dist),
        );
        self.horizontal = multi(self.u, 2.0 * half_width * camera.focus_dist);
        self.vertical = multi(self.v, 2.0 * half_height * camera.focus_dist);
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = multi(random_in_unit_disk(), self.lens_radius);
        let offset = add(multi(self.u, rd.0), multi(self.v, rd.1));

        Ray {
            ori: add(self.origin, offset),
            dir: reduce(
                reduce(
                    add(
                        add(self.lower_left_corner, multi(self.horizontal, s)),
                        multi(self.vertical, t),
                    ),
                    self.origin,
                ),
                offset,
            ),
            tm: random_double(self.time0, self.time1),
        }
    }
}
