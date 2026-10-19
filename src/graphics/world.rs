use super::{architecture, geometry::Geometry, terrain};
use crate::{
    game::{Pilot, combat::Threats, mission::Mission},
    level::{HOME, ISLANDS, RELICS},
};
use bevy::prelude::*;

#[derive(Component)]
pub(super) enum Visual {
    Relic(usize),
    Hunter(usize),
    Pulse(usize),
    Beacon,
    HomeHalo,
    Rope(usize),
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut surfaces: ResMut<Assets<super::surface::IslandMaterial>>,
    assets: Res<AssetServer>,
) {
    let texture = super::surface::load(&assets, &mut commands);
    let island_material = surfaces.add(super::surface::IslandMaterial {
        base: StandardMaterial {
            perceptual_roughness: 0.94,
            ..default()
        },
        extension: super::surface::RockSurface {
            albedo: texture,
            parameters: Vec4::new(0.075, 1.0, 0.94, 0.0),
        },
    });
    let stone = materials.add(StandardMaterial {
        perceptual_roughness: 0.91,
        ..default()
    });
    let leaves = materials.add(StandardMaterial {
        perceptual_roughness: 0.96,
        cull_mode: None,
        ..default()
    });
    for (index, island) in ISLANDS.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(meshes.add(terrain::island(island).mesh())),
            MeshMaterial3d(island_material.clone()),
        ));
        commands.spawn((
            Mesh3d(meshes.add(terrain::vegetation(island).mesh())),
            MeshMaterial3d(leaves.clone()),
        ));
        if index > 0 && index < 4 {
            commands.spawn((
                Mesh3d(meshes.add(architecture::ruins(island, index).mesh())),
                MeshMaterial3d(stone.clone()),
            ));
        }
    }
    commands.spawn((
        Mesh3d(meshes.add(architecture::harbor().mesh())),
        MeshMaterial3d(stone.clone()),
    ));
    commands.spawn((
        Mesh3d(meshes.add(architecture::broken_ring().mesh())),
        MeshMaterial3d(stone),
    ));
    let bronze = materials.add(StandardMaterial {
        base_color: Color::srgb(0.73, 0.5, 0.22),
        metallic: 0.7,
        perceptual_roughness: 0.3,
        ..default()
    });
    let mut ring = Geometry::default();
    ring.arc(
        Vec3::ZERO,
        2.7,
        0.13,
        0.2,
        0.0,
        std::f32::consts::TAU,
        Quat::IDENTITY,
        [1.0, 1.0, 1.0],
    );
    let ring = meshes.add(ring.mesh());
    for (index, relic) in RELICS.iter().enumerate() {
        let color = relic.color;
        let glow = materials.add(StandardMaterial {
            base_color: Color::srgb(color[0], color[1], color[2]),
            emissive: LinearRgba::rgb(color[0] * 3.0, color[1] * 3.0, color[2] * 3.0),
            metallic: 0.3,
            perceptual_roughness: 0.16,
            ..default()
        });
        commands
            .spawn((
                Visual::Relic(index),
                Transform::from_translation(relic.position),
                Visibility::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Mesh3d(meshes.add(Sphere::new(1.55).mesh().ico(0).unwrap())),
                    MeshMaterial3d(glow),
                ));
                for axis in [
                    Quat::IDENTITY,
                    Quat::from_rotation_x(1.3),
                    Quat::from_rotation_y(1.3),
                ] {
                    parent.spawn((
                        Mesh3d(ring.clone()),
                        MeshMaterial3d(bronze.clone()),
                        Transform::from_rotation(axis),
                    ));
                }
            });
        let island = ISLANDS[index + 1];
        let base = Vec3::new(
            relic.position.x,
            island.surface(relic.position.x, relic.position.z),
            relic.position.z,
        );
        let height = relic.position.y - base.y - 3.0;
        let mut pedestal = Geometry::default();
        pedestal.cylinder(
            base + Vec3::Y * height * 0.5,
            3.1,
            height,
            12,
            [0.63, 0.58, 0.43],
        );
        pedestal.cylinder(base + Vec3::Y * height, 4.8, 1.0, 24, [0.7, 0.65, 0.46]);
        commands.spawn((
            Mesh3d(meshes.add(pedestal.mesh())),
            MeshMaterial3d(bronze.clone()),
        ));
    }
    let beacon_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.45, 0.95, 0.8, 0.12),
        emissive: LinearRgba::rgb(0.3, 1.2, 0.9),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.spawn((
        Visual::Beacon,
        Mesh3d(meshes.add(Cylinder::new(0.65, 65.0))),
        MeshMaterial3d(beacon_material),
        Transform::default(),
        bevy::light::NotShadowCaster,
    ));
    let home_glow = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.7, 0.3),
        emissive: LinearRgba::rgb(3.5, 2.4, 0.5),
        ..default()
    });
    commands.spawn((
        Visual::HomeHalo,
        Mesh3d(ring.clone()),
        MeshMaterial3d(home_glow),
        Transform::from_translation(HOME).with_scale(Vec3::splat(4.3)),
        bevy::light::NotShadowCaster,
    ));
    let rope_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.19, 0.1),
        ..default()
    });
    let rope_mesh = meshes.add(Cylinder::new(0.045, 1.0));
    for index in 0..12 {
        commands.spawn((
            Visual::Rope(index),
            Mesh3d(rope_mesh.clone()),
            MeshMaterial3d(rope_material.clone()),
            Transform::default(),
            Visibility::Hidden,
        ));
    }
    let mut hunter = Geometry::default();
    hunter.ellipsoid(Vec3::ZERO, Vec3::new(2.2, 0.8, 2.5), [0.16, 0.2, 0.2], 3.0);
    for side in [-1.0, 1.0] {
        hunter.cuboid(
            Vec3::new(side * 2.6, 0.0, 0.3),
            Vec3::new(3.0, 0.25, 1.2),
            Quat::from_rotation_z(side * 0.2),
            [0.3, 0.3, 0.22],
        );
        hunter.arc(
            Vec3::new(side * 3.5, 0.0, 0.3),
            1.2,
            0.16,
            0.2,
            0.0,
            std::f32::consts::TAU,
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            [0.48, 0.32, 0.15],
        );
    }
    let hunter_mesh = meshes.add(hunter.mesh());
    let hunter_material = materials.add(StandardMaterial {
        metallic: 0.65,
        perceptual_roughness: 0.4,
        ..default()
    });
    let enemy_glow = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.2, 0.03),
        emissive: LinearRgba::rgb(5.0, 0.2, 0.03),
        ..default()
    });
    let sphere = meshes.add(Sphere::new(0.5).mesh().uv(10, 6));
    for index in 0..3 {
        commands
            .spawn((
                Visual::Hunter(index),
                Mesh3d(hunter_mesh.clone()),
                MeshMaterial3d(hunter_material.clone()),
                Transform::default(),
            ))
            .with_children(|p| {
                p.spawn((
                    Mesh3d(sphere.clone()),
                    MeshMaterial3d(enemy_glow.clone()),
                    Transform::from_xyz(0.0, 0.25, -2.3),
                ));
            });
    }
    let pulse_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.8, 1.0),
        emissive: LinearRgba::rgb(2.0, 5.0, 7.0),
        unlit: true,
        ..default()
    });
    for index in 0..64 {
        commands.spawn((
            Visual::Pulse(index),
            Mesh3d(sphere.clone()),
            MeshMaterial3d(pulse_material.clone()),
            Transform::default(),
            Visibility::Hidden,
            bevy::light::NotShadowCaster,
        ));
    }
}

pub(super) fn animate(
    time: Res<Time>,
    pilot: Res<Pilot>,
    mission: Res<Mission>,
    threats: Res<Threats>,
    mut visuals: Query<(&Visual, &mut Transform, &mut Visibility)>,
) {
    let now = time.elapsed_secs();
    for (kind, mut transform, mut visibility) in &mut visuals {
        match *kind {
            Visual::Relic(index) => {
                *visibility = if index < mission.delivered {
                    Visibility::Hidden
                } else {
                    Visibility::Inherited
                };
                transform.translation = if let Some(payload) = &mission.payload {
                    if payload.relic == index {
                        payload.position
                    } else {
                        mission.loose[index]
                    }
                } else {
                    mission.loose[index]
                };
                transform.rotation = Quat::from_euler(EulerRot::YXZ, now * 0.24, now * 0.15, 0.3);
            }
            Visual::Beacon => {
                transform.translation = mission.target() + Vec3::Y * 29.0;
                *visibility = if mission.delivered < 3 {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
            Visual::HomeHalo => {
                transform.rotation = Quat::from_rotation_z(now * 0.12);
            }
            Visual::Hunter(index) => {
                let hunter = &threats.hunters[index];
                transform.translation = hunter.position;
                if hunter.health > 0.0 && hunter.velocity.length() > 0.1 {
                    transform.look_to(hunter.velocity.normalize(), Vec3::Y);
                } else if hunter.health <= 0.0 {
                    transform.rotate_local_z(time.delta_secs() * 1.5);
                }
                *visibility = if hunter.position.y < -80.0 {
                    Visibility::Hidden
                } else {
                    Visibility::Inherited
                };
            }
            Visual::Pulse(index) => {
                if let Some(pulse) = threats.pulses.get(index) {
                    *visibility = Visibility::Inherited;
                    transform.translation = pulse.position;
                    transform.look_to(pulse.velocity.normalize_or_zero(), Vec3::Y);
                    transform.scale = Vec3::new(0.4, 0.4, 2.8);
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
            Visual::Rope(index) => {
                if let Some(payload) = &mission.payload {
                    *visibility = Visibility::Inherited;
                    let anchor = pilot.body.position - Vec3::Y * 2.0;
                    let end = payload.position;
                    let a = anchor.lerp(end, index as f32 / 12.0);
                    let b = anchor.lerp(end, (index + 1) as f32 / 12.0);
                    let delta = b - a;
                    transform.translation = (a + b) * 0.5;
                    transform.rotation =
                        Quat::from_rotation_arc(Vec3::Y, delta.normalize_or_zero());
                    transform.scale = Vec3::new(1.0, delta.length(), 1.0);
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
