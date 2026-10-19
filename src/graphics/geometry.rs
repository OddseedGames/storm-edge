use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

#[derive(Default)]
pub struct Geometry {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl Geometry {
    pub fn triangle(&mut self, a: Vec3, b: Vec3, c: Vec3, color: [f32; 3]) {
        let index = self.positions.len() as u32;
        let normal = (b - a).cross(c - a).normalize_or_zero().to_array();
        for p in [a, b, c] {
            self.positions.push(p.to_array());
            self.normals.push(normal);
            self.colors.push(
                Color::srgb(color[0], color[1], color[2])
                    .to_linear()
                    .to_f32_array(),
            );
            self.uvs.push([p.x * 0.05, p.z * 0.05]);
        }
        self.indices
            .extend_from_slice(&[index, index + 1, index + 2]);
    }

    pub fn quad(&mut self, a: Vec3, b: Vec3, c: Vec3, d: Vec3, color: [f32; 3]) {
        self.triangle(a, b, c, color);
        self.triangle(a, c, d, color);
    }

    pub fn cuboid(&mut self, center: Vec3, size: Vec3, rotation: Quat, color: [f32; 3]) {
        let half = size * 0.5;
        let vertex = |x: f32, y: f32, z: f32| center + rotation * (half * Vec3::new(x, y, z));
        let v = [
            vertex(-1., -1., -1.),
            vertex(1., -1., -1.),
            vertex(1., 1., -1.),
            vertex(-1., 1., -1.),
            vertex(-1., -1., 1.),
            vertex(1., -1., 1.),
            vertex(1., 1., 1.),
            vertex(-1., 1., 1.),
        ];
        for [a, b, c, d] in [
            [0, 3, 2, 1],
            [4, 5, 6, 7],
            [0, 4, 7, 3],
            [1, 2, 6, 5],
            [3, 7, 6, 2],
            [0, 1, 5, 4],
        ] {
            self.quad(v[a], v[b], v[c], v[d], color);
        }
    }

    pub fn beam(&mut self, a: Vec3, b: Vec3, width: f32, color: [f32; 3]) {
        let delta = b - a;
        self.cuboid(
            (a + b) * 0.5,
            Vec3::new(width, delta.length(), width),
            Quat::from_rotation_arc(Vec3::Y, delta.normalize_or_zero()),
            color,
        );
    }

    pub fn cylinder(
        &mut self,
        center: Vec3,
        radius: f32,
        height: f32,
        sides: usize,
        color: [f32; 3],
    ) {
        let top = center + Vec3::Y * height * 0.5;
        let bottom = center - Vec3::Y * height * 0.5;
        for i in 0..sides {
            let a = i as f32 * std::f32::consts::TAU / sides as f32;
            let b = (i + 1) as f32 * std::f32::consts::TAU / sides as f32;
            let u = Vec3::new(a.cos() * radius, 0.0, a.sin() * radius);
            let v = Vec3::new(b.cos() * radius, 0.0, b.sin() * radius);
            self.quad(bottom + u, top + u, top + v, bottom + v, color);
            self.triangle(top, top + v, top + u, color);
            self.triangle(bottom, bottom + u, bottom + v, color);
        }
    }

    pub fn arc(
        &mut self,
        center: Vec3,
        radius: f32,
        thickness: f32,
        depth: f32,
        start: f32,
        end: f32,
        rotation: Quat,
        color: [f32; 3],
    ) {
        let segments = ((end - start).abs() * radius * 0.6).max(8.0) as usize;
        for i in 0..segments {
            let angle = start + (end - start) * (i as f32 + 0.5) / segments as f32;
            let local = Vec3::new(angle.cos() * radius, angle.sin() * radius, 0.0);
            self.cuboid(
                center + rotation * local,
                Vec3::new(
                    thickness,
                    radius * (end - start).abs() / segments as f32 * 1.04,
                    depth,
                ),
                rotation * Quat::from_rotation_z(angle),
                color,
            );
        }
    }

    pub fn ellipsoid(&mut self, center: Vec3, scale: Vec3, color: [f32; 3], seed: f32) {
        let rings = 7;
        let sides = 12;
        let point = |r: usize, s: usize| {
            let phi = r as f32 / rings as f32 * std::f32::consts::PI;
            let theta = s as f32 / sides as f32 * std::f32::consts::TAU;
            let noise =
                0.92 + crate::level::hash(r as f32 * 13.0 + (s % sides) as f32 + seed) * 0.16;
            center
                + scale
                    * Vec3::new(phi.sin() * theta.cos(), phi.cos(), phi.sin() * theta.sin())
                    * noise
        };
        for r in 0..rings {
            for s in 0..sides {
                let k = 0.86 + crate::level::hash(r as f32 * 17.0 + s as f32 + seed) * 0.25;
                let tint = [color[0] * k, color[1] * k, color[2] * k];
                self.quad(
                    point(r, s),
                    point(r, s + 1),
                    point(r + 1, s + 1),
                    point(r + 1, s),
                    tint,
                );
            }
        }
    }

    pub fn smooth(mut self) -> Self {
        let key = |p: [f32; 3]| {
            (
                (p[0] * 1000.0).round() as i32,
                (p[1] * 1000.0).round() as i32,
                (p[2] * 1000.0).round() as i32,
            )
        };
        let mut normals = std::collections::HashMap::<(i32, i32, i32), Vec3>::new();
        for (point, normal) in self.positions.iter().zip(&self.normals) {
            *normals.entry(key(*point)).or_default() += Vec3::from_array(*normal);
        }
        for (point, normal) in self.positions.iter().zip(&mut self.normals) {
            *normal = normals[&key(*point)].normalize_or_zero().to_array();
        }
        self
    }

    pub fn mesh(self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.colors)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs)
        .with_inserted_indices(Indices::U32(self.indices))
    }
}
