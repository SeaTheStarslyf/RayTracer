use crate::material::*;
//use crate::ray::*;
use crate::shape::*;
use crate::vec3::*;
//use image::imageops::FilterType::Triangle;
use std::sync::Arc;
use tobj::{load_obj, LoadOptions};

type Object = (Arc<dyn Material>, Arc<dyn Shape>);

pub fn getobject(v: &mut Vec<Object>, filename: &str) {
    let (models, _) = load_obj(filename, &LoadOptions::default()).unwrap();

    let x = -100.0;
    let y = 100.0;
    // 遍历所有模型
    for model in models {
        let mesh = &model.mesh;

        // 遍历每个面的顶点索引
        for face in (0..mesh.indices.len() - 2).step_by(3) {
            // 获取三角形的顶点索引
            //                let vertex_indices = (face[0], face[1], face[2]);

            // 获取三角形的顶点位置
            let vertex_indices = (
                mesh.indices[face],
                mesh.indices[face + 1],
                mesh.indices[face + 2],
            );

            let positions = (
                Vec3(
                    mesh.positions[(vertex_indices.0 * 3) as usize] as f64 + x,
                    mesh.positions[(vertex_indices.0 * 3 + 1) as usize] as f64 + y,
                    mesh.positions[(vertex_indices.0 * 3 + 2) as usize] as f64,
                ),
                Vec3(
                    mesh.positions[(vertex_indices.1 * 3) as usize] as f64 + x,
                    mesh.positions[(vertex_indices.1 * 3 + 1) as usize] as f64 + y,
                    mesh.positions[(vertex_indices.1 * 3 + 2) as usize] as f64,
                ),
                Vec3(
                    mesh.positions[(vertex_indices.2 * 3) as usize] as f64 + x,
                    mesh.positions[(vertex_indices.2 * 3 + 1) as usize] as f64 + y,
                    mesh.positions[(vertex_indices.2 * 3 + 2) as usize] as f64,
                ),
            );
            let vx = positions.0;
            let vy = positions.1;
            let vz = positions.2;

            // 获取三角形的顶点颜色
            /*           let color_indices = (
                vertex_indices.0 as usize,
                vertex_indices.1 as usize,
                vertex_indices.2 as usize,
            );
            let colors = (
                Vec3(
                    mesh.vertex_color[color_indices.0 * 3] as f64 / 255.0,
                    mesh.vertex_color[color_indices.0 * 3 + 1] as f64 / 255.0,
                    mesh.vertex_color[color_indices.0 * 3 + 2] as f64 / 255.0,
                ),
                Vec3(
                    mesh.vertex_color[color_indices.1 * 3] as f64 / 255.0,
                    mesh.vertex_color[color_indices.1 * 3 + 1] as f64 / 255.0,
                    mesh.vertex_color[color_indices.1 * 3 + 2] as f64 / 255.0,
                ),
                Vec3(
                    mesh.vertex_color[color_indices.2 * 3] as f64 / 255.0,
                    mesh.vertex_color[color_indices.2 * 3 + 1] as f64 / 255.0,
                    mesh.vertex_color[color_indices.2 * 3 + 2] as f64 / 255.0,
                ),
            );
            let color1 = Vec3(
                (colors.0 .0 + colors.1 .0 + colors.2 .0) / 3.0,
                (colors.0 .1 + colors.1 .1 + colors.2 .1) / 3.0,
                (colors.0 .2 + colors.1 .2 + colors.2 .2) / 3.0,
            );*/
            let color1 = Vec3(1.0, 0.0, 0.0);

            let triangle = Triangle1 {
                v0: vx,
                v1: vy,
                v2: vz,
            };
            let objlambertian = Objlambertian { color: color1 };
            v.push((Arc::new(objlambertian), Arc::new(triangle)));
        }
    }
}
