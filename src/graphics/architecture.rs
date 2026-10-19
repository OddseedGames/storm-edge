use super::geometry::Geometry;
use crate::level::{HOME, Island, hash};
use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

pub fn ruins(island: Island, index: usize) -> Geometry {
    let mut mesh = Geometry::default();
    let stone = [0.64, 0.57, 0.41];
    let pale = [0.77, 0.7, 0.52];
    let dark = [0.32, 0.37, 0.32];
    let p = island.center + Vec3::Y * 6.0;
    mesh.cylinder(p - Vec3::Y, island.radius * 0.36, 3.0, 32, dark);
    mesh.cylinder(p + Vec3::Y * 0.7, island.radius * 0.31, 0.9, 32, stone);
    let count = if index == 3 { 12 } else { 7 };
    for i in 0..count {
        let angle = i as f32 / count as f32 * TAU;
        let radius = island.radius * 0.28;
        let point = p + Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius);
        let height = 9.0 + hash(i as f32 + island.seed) * 15.0;
        mesh.cylinder(point + Vec3::Y * height * 0.5, 1.6, height, 10, stone);
        mesh.cylinder(point + Vec3::Y * (height + 0.2), 2.2, 0.9, 10, pale);
        mesh.cylinder(point, 2.2, 1.4, 10, pale);
        for step in 1..4 {
            mesh.cylinder(
                point + Vec3::Y * (height * step as f32 / 4.0),
                1.72,
                0.28,
                10,
                pale,
            );
        }
        if i % 3 != 0 {
            let next = (i + 1) as f32 / count as f32 * TAU;
            let q = p + Vec3::new(next.cos() * radius, height, next.sin() * radius);
            mesh.beam(point + Vec3::Y * height, q, 1.4, stone);
        }
    }
    if index == 1 || index == 2 {
        let center = p + Vec3::new(-island.radius * 0.55, 16.0, 0.0);
        mesh.arc(
            center,
            16.0,
            2.4,
            4.0,
            -0.3,
            PI + 0.3,
            Quat::from_rotation_y(0.3),
            pale,
        );
        mesh.cuboid(
            center + Vec3::new(-16.0, -9.0, 0.0),
            Vec3::new(3.0, 18.0, 4.0),
            Quat::IDENTITY,
            stone,
        );
        mesh.cuboid(
            center + Vec3::new(16.0, -9.0, 0.0),
            Vec3::new(3.0, 18.0, 4.0),
            Quat::IDENTITY,
            stone,
        );
    }
    for i in 0..14 {
        let k = i as f32 + island.seed;
        let x = (hash(k * 2.0) - 0.5) * island.radius * 1.5;
        let z = (hash(k * 2.0 + 9.0) - 0.5) * island.radius * 1.5;
        let point = Vec3::new(p.x + x, island.surface(p.x + x, p.z + z), p.z + z);
        mesh.cuboid(
            point,
            Vec3::new(2.0 + hash(k) * 3.0, 1.3, 2.1),
            Quat::from_euler(EulerRot::YXZ, k, hash(k) * 0.4, hash(k + 1.0) * 0.4),
            stone,
        );
    }
    mesh
}

pub fn harbor() -> Geometry {
    let mut mesh = Geometry::default();
    let wood = [0.32, 0.25, 0.18];
    let brass = [0.56, 0.39, 0.2];
    let pale = [0.75, 0.66, 0.48];
    let p = HOME - Vec3::Y * 16.0;
    mesh.cuboid(p, Vec3::new(28.0, 2.0, 42.0), Quat::IDENTITY, wood);
    for i in -9..10 {
        mesh.cuboid(
            p + Vec3::new(i as f32 * 1.4, 1.1, 0.0),
            Vec3::new(0.12, 0.1, 40.0),
            Quat::IDENTITY,
            brass,
        );
    }
    for side in [-1.0, 1.0] {
        for i in -3..4 {
            let x = side * 13.0;
            let z = i as f32 * 6.0;
            mesh.cuboid(
                p + Vec3::new(x, -5.0, z),
                Vec3::new(1.5, 12.0, 1.5),
                Quat::IDENTITY,
                wood,
            );
            mesh.cuboid(
                p + Vec3::new(x, 3.0, z),
                Vec3::new(0.45, 4.0, 0.45),
                Quat::IDENTITY,
                brass,
            );
        }
        mesh.beam(
            p + Vec3::new(side * 13.0, 4.5, -20.0),
            p + Vec3::new(side * 13.0, 4.5, 20.0),
            0.2,
            brass,
        );
    }
    let building = p + Vec3::new(0.0, 7.5, 26.0);
    mesh.cuboid(building, Vec3::new(19.0, 15.0, 16.0), Quat::IDENTITY, pale);
    for side in [-1.0, 1.0] {
        mesh.cuboid(
            building + Vec3::new(side * 5.1, 10.5, 0.0),
            Vec3::new(12.0, 0.8, 20.0),
            Quat::from_rotation_z(side * 0.5),
            [0.18, 0.35, 0.33],
        );
    }
    mesh.cylinder(building + Vec3::new(0.0, 19.0, 4.0), 2.6, 25.0, 12, pale);
    mesh.cylinder(building + Vec3::new(0.0, 31.0, 4.0), 4.0, 1.6, 12, brass);
    mesh.arc(
        HOME + Vec3::new(0.0, 0.0, -12.0),
        14.0,
        0.5,
        0.5,
        0.0,
        TAU,
        Quat::IDENTITY,
        brass,
    );
    mesh
}

pub fn broken_ring() -> Geometry {
    let mut mesh = Geometry::default();
    let center = Vec3::new(130.0, 145.0, -520.0);
    let rotation = Quat::from_rotation_y(-0.45) * Quat::from_rotation_z(0.28);
    for (start, end) in [(0.12, 1.42), (1.51, 3.0), (3.1, 4.6), (4.78, 5.97)] {
        mesh.arc(
            center,
            185.0,
            7.0,
            14.0,
            start,
            end,
            rotation,
            [0.42, 0.49, 0.46],
        );
        mesh.arc(
            center,
            191.0,
            1.3,
            17.0,
            start,
            end,
            rotation,
            [0.68, 0.61, 0.42],
        );
        mesh.arc(
            center,
            178.0,
            1.7,
            17.0,
            start,
            end,
            rotation,
            [0.68, 0.61, 0.42],
        );
    }
    for i in 0..55 {
        let angle = i as f32 / 55.0 * TAU;
        let p = center + rotation * Vec3::new(angle.cos() * 185.0, angle.sin() * 185.0, 0.0);
        mesh.cuboid(
            p,
            Vec3::new(19.0, 2.0, 20.0),
            rotation * Quat::from_rotation_z(angle),
            [0.53, 0.55, 0.45],
        );
    }
    mesh
}
