use crate::tool::*;
use crate::vec3::*;

const POINT_COUNT: i32 = 256;

#[derive(Clone)]
pub struct Perlin {
    pub ranvec: [Vec3; POINT_COUNT as usize],
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
            self.ranvec[i as usize] = unit_vector(random_vector(-1.0, 1.0));
        }
        self.perm_x = perlin_generate_perm();
        self.perm_y = perlin_generate_perm();
        self.perm_z = perlin_generate_perm();
    }
    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.0 - p.0.floor();
        let v = p.1 - p.1.floor();
        let w = p.2 - p.2.floor();
        let i = p.0.floor() as i32;
        let j = p.1.floor() as i32;
        let k = p.2.floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3(0.0, 0.0, 0.0); 2]; 2]; 2];

        for (di, row1) in c.iter_mut().enumerate() {
            for (dj, row2) in row1.iter_mut().enumerate().take(2) {
                for (dk, row3) in row2.iter_mut().enumerate().take(2) {
                    *row3 = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, _) in c.iter().enumerate() {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * dot(c[i][j][k], weight_v);
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = multi(temp_p, 2.0);
        }

        accum.abs()
    }
}
