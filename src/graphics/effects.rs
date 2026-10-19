use super::geometry::Geometry;
use crate::{game::Pilot, level::hash};
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Bird {
    pub index: usize,
}
#[derive(Component)]
pub struct Debris {
    pub index: usize,
}
#[derive(Resource, Default)]
pub struct Trails {
    pub left: VecDeque<Vec3>,
    pub right: VecDeque<Vec3>,
    pub last: Vec3,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.init_resource::<Trails>();
    let mut bird = Geometry::default();
    bird.triangle(
        Vec3::new(-0.7, 0.1, 0.3),
        Vec3::new(0.0, 0.0, -0.4),
        Vec3::new(0.0, -0.03, 0.3),
        [0.18, 0.24, 0.23],
    );
    bird.triangle(
        Vec3::new(0.0, -0.03, 0.3),
        Vec3::new(0.0, 0.0, -0.4),
        Vec3::new(0.7, 0.1, 0.3),
        [0.18, 0.24, 0.23],
    );
    let bird = meshes.add(bird.mesh());
    let mat = materials.add(StandardMaterial {
        cull_mode: None,
        ..default()
    });
    for index in 0..28 {
        commands.spawn((
            Bird { index },
            Mesh3d(bird.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::default(),
            bevy::light::NotShadowCaster,
        ));
    }
    let mut chunk = Geometry::default();
    chunk.ellipsoid(Vec3::ZERO, Vec3::ONE, [0.41, 0.46, 0.4], 3.0);
    let chunk = meshes.add(chunk.mesh());
    for index in 0..26 {
        let angle = hash(index as f32) * std::f32::consts::TAU;
        let p = Vec3::new(
            130.0 + angle.cos() * 190.0,
            145.0 + angle.sin() * 190.0,
            -520.0 + hash(index as f32 + 7.0) * 90.0,
        );
        commands.spawn((
            Debris { index },
            Mesh3d(chunk.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(p).with_scale(Vec3::new(
                1.0 + hash(index as f32 + 4.0) * 7.0,
                2.0 + hash(index as f32 + 8.0) * 8.0,
                2.0,
            )),
        ));
    }
    let mut horizon = Geometry::default();
    for index in 0..32 {
        let angle = index as f32 / 32.0 * std::f32::consts::TAU;
        let radius = 850.0 + hash(index as f32) * 450.0;
        horizon.ellipsoid(
            Vec3::new(
                angle.cos() * radius,
                -40.0 + hash(index as f32 + 1.0) * 120.0,
                angle.sin() * radius - 120.0,
            ),
            Vec3::new(
                40.0 + hash(index as f32 + 3.0) * 65.0,
                120.0 + hash(index as f32 + 8.0) * 160.0,
                40.0,
            ),
            [0.32, 0.43, 0.4],
            index as f32,
        );
    }
    commands.spawn((Mesh3d(meshes.add(horizon.mesh())), MeshMaterial3d(mat)));
}

pub fn animate(
    time: Res<Time>,
    pilot: Res<Pilot>,
    mut trails: ResMut<Trails>,
    mut gizmos: Gizmos,
    mut birds: Query<(&Bird, &mut Transform), Without<Debris>>,
    mut debris: Query<(&Debris, &mut Transform), Without<Bird>>,
) {
    let now = time.elapsed_secs();
    for (bird, mut transform) in &mut birds {
        let i = bird.index as f32;
        let angle = now * 0.08 + i * 0.13;
        transform.translation = Vec3::new(
            -93.0 + angle.cos() * 62.0,
            66.0 + (now * 0.6 + i).sin() * 3.0 + i * 0.25,
            angle.sin() * 44.0 - 42.0,
        );
        transform.rotation = Quat::from_rotation_y(-angle - std::f32::consts::FRAC_PI_2)
            * Quat::from_rotation_z((now * 4.0 + i).sin() * 0.2);
        transform.scale.y = 0.5 + (now * 5.0 + i).sin().abs() * 1.4;
    }
    for (debris, mut transform) in &mut debris {
        let i = debris.index as f32;
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            now * (0.03 + i * 0.001) + i,
            now * 0.02 + i,
            i,
        );
    }
    let body = &pilot.body;
    if body.position.distance(trails.last) > 0.12 {
        let rotation = body.rotation();
        trails
            .left
            .push_back(body.position + rotation * Vec3::new(-5.4, 1.4, 1.0));
        trails
            .right
            .push_back(body.position + rotation * Vec3::new(5.4, 1.4, 1.0));
        trails.last = body.position;
    }
    while trails.left.len() > 65 {
        trails.left.pop_front();
        trails.right.pop_front();
    }
    if body.velocity.length() > 8.0 {
        for trail in [&trails.left, &trails.right] {
            for index in 1..trail.len() {
                let alpha = index as f32 / trail.len() as f32 * 0.33;
                gizmos.line(
                    trail[index - 1],
                    trail[index],
                    Color::srgba(0.77, 0.91, 0.89, alpha),
                );
            }
        }
    }
}
