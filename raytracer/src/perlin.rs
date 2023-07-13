use crate::tool::*;
use crate::vec3::*;

const POINT_COUNT: i32 = 256;

#[derive(Clone)]
pub struct Perlin {
    pub ranfloat: [f64; POINT_COUNT as usize],
    pub perm_x: [i32; POINT_COUNT as usize],
    pub perm_y: [i32; POINT_COUNT as usize],
    pub perm_z: [i32; POINT_COUNT as usize],
}

fn perlin_generate_perm() -> [i32; POINT_COUNT as usize] {
    let mut p: [i32; POINT_COUNT as usize] = [0; POINT_COUNT as usize];
    for i in 0..POINT_COUNT {
        p[i as usize] = i;
    }
    for i in (0..(POINT_COUNT - 1)).rev() {
        let target = random_int(0, i);
        p.swap(i as usize, target as usize);
    }
    p
}

impl Perlin {
    pub fn build(&mut self) {
        for i in 0..POINT_COUNT {
            self.ranfloat[i as usize] = random_double(0.0, 1.0);
        }
        self.perm_x = perlin_generate_perm();
        self.perm_y = perlin_generate_perm();
        self.perm_z = perlin_generate_perm();
    }
    pub fn noise(&self, p: Vec3) -> f64 {
        let i = (4.0 * p.0) as i32 & 255;
        let j = (4.0 * p.1) as i32 & 255;
        let k = (4.0 * p.2) as i32 & 255;
        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}
