use super::geometry::Geometry;
use crate::game::Pilot;
use bevy::prelude::*;

#[derive(Component)]
pub struct Ship;
#[derive(Component)]
pub struct Propeller;
#[derive(Component)]
pub struct Exhaust;
#[derive(Component)]
pub struct Shield;
#[derive(Component)]
pub struct Sail {
    pub mesh: Handle<Mesh>,
    pub original: Vec<[f32; 3]>,
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut hull = Geometry::default();
    let mut metal = Geometry::default();
    let mut fabric = Geometry::default();
    let bronze = [0.52, 0.33, 0.15];
    let wood = [0.23, 0.15, 0.09];
    let teal = [0.08, 0.28, 0.29];
    let hull_point = |ring: usize, side: usize| {
        let z = -3.6 + ring as f32 * 0.36;
        let width = (1.0 - (z / 3.7).powi(2)).max(0.01).sqrt();
        let angle = side as f32 / 16.0 * std::f32::consts::TAU;
        Vec3::new(angle.cos() * width * 1.3, angle.sin() * width * 0.85, z)
    };
    for ring in 0..20 {
        for side in 0..16 {
            let color = if side % 4 == 0 {
                bronze
            } else if side > 7 {
                wood
            } else {
                teal
            };
            hull.quad(
                hull_point(ring, side),
                hull_point(ring, side + 1),
                hull_point(ring + 1, side + 1),
                hull_point(ring + 1, side),
                color,
            );
        }
    }
    for side in 0..16 {
        hull.triangle(
            Vec3::new(0.0, 0.0, -3.6),
            hull_point(0, side + 1),
            hull_point(0, side),
            teal,
        );
        hull.triangle(
            Vec3::new(0.0, 0.0, 3.6),
            hull_point(20, side),
            hull_point(20, side + 1),
            teal,
        );
    }
    for ring in [3, 6, 9, 12, 15, 18] {
        for side in 0..16 {
            let a = hull_point(ring, side) * Vec3::new(1.025, 1.025, 1.0);
            let b = hull_point(ring, side + 1) * Vec3::new(1.025, 1.025, 1.0);
            metal.beam(a, b, 0.055, bronze);
        }
    }
    for side in [-1.0, 1.0] {
        metal.beam(
            Vec3::new(side * 0.9, 0.25, 1.5),
            Vec3::new(side * 2.4, 0.0, 1.7),
            0.16,
            bronze,
        );
        metal.beam(
            Vec3::new(side * 0.9, 0.25, -1.2),
            Vec3::new(side * 2.4, 0.0, 0.5),
            0.13,
            bronze,
        );
        hull.ellipsoid(
            Vec3::new(side * 2.4, 0.0, 1.1),
            Vec3::new(0.48, 0.48, 1.7),
            teal,
            side + 4.0,
        );
        metal.arc(
            Vec3::new(side * 2.4, 0.0, 2.25),
            0.8,
            0.08,
            0.13,
            0.0,
            std::f32::consts::TAU,
            Quat::IDENTITY,
            bronze,
        );
        metal.beam(
            Vec3::new(0.0, 3.8, 1.0),
            Vec3::new(side * 5.4, 1.35, 0.7),
            0.045,
            wood,
        );
        metal.beam(
            Vec3::new(0.0, 0.75, -1.9),
            Vec3::new(side * 5.4, 1.35, 0.7),
            0.065,
            bronze,
        );
        metal.beam(
            Vec3::new(0.0, 1.6, 1.4),
            Vec3::new(side * 5.4, 1.35, 0.7),
            0.09,
            bronze,
        );
        for x in 0..12 {
            for z in 0..6 {
                let point = |ix: usize, iz: usize| {
                    let u = ix as f32 / 12.0;
                    let v = iz as f32 / 6.0;
                    Vec3::new(
                        side * (0.7 + u * 4.9),
                        1.7 + u * 0.2 - (v * std::f32::consts::PI).sin() * 0.45,
                        -1.9 + u * 2.2 + v * (3.4 - u * 1.1),
                    )
                };
                let color = if x == 11 || z == 0 || z == 5 {
                    [0.8, 0.56, 0.25]
                } else if x % 4 == 0 {
                    [0.65, 0.19, 0.12]
                } else {
                    [0.48, 0.09, 0.065]
                };
                if side > 0.0 {
                    fabric.quad(
                        point(x, z),
                        point(x, z + 1),
                        point(x + 1, z + 1),
                        point(x + 1, z),
                        color,
                    );
                } else {
                    fabric.quad(
                        point(x, z),
                        point(x + 1, z),
                        point(x + 1, z + 1),
                        point(x, z + 1),
                        color,
                    );
                }
            }
        }
    }
    metal.beam(
        Vec3::new(0.0, 0.6, 1.2),
        Vec3::new(0.0, 3.9, 1.2),
        0.14,
        bronze,
    );
    fabric.triangle(
        Vec3::new(0.0, 3.85, 1.2),
        Vec3::new(0.0, 1.4, 1.2),
        Vec3::new(0.0, 1.5, 3.4),
        [0.65, 0.16, 0.09],
    );
    metal.cuboid(
        Vec3::new(0.0, 0.65, -0.6),
        Vec3::new(1.35, 0.25, 1.7),
        Quat::IDENTITY,
        wood,
    );
    hull.ellipsoid(
        Vec3::new(0.0, 0.98, -0.15),
        Vec3::new(0.39, 0.54, 0.32),
        [0.22, 0.23, 0.18],
        0.0,
    );
    hull.ellipsoid(
        Vec3::new(0.0, 1.55, -0.15),
        Vec3::splat(0.26),
        [0.53, 0.34, 0.18],
        0.0,
    );
    metal.cuboid(
        Vec3::new(0.0, 1.54, -0.36),
        Vec3::new(0.37, 0.14, 0.12),
        Quat::IDENTITY,
        [0.08, 0.17, 0.17],
    );
    for side in [-1.0, 1.0] {
        metal.beam(
            Vec3::new(side * 0.22, 1.1, -0.2),
            Vec3::new(side * 0.35, 1.1, -0.8),
            0.11,
            [0.31, 0.28, 0.19],
        );
    }
    let mut glass = Geometry::default();
    glass.quad(
        Vec3::new(-0.7, 0.8, -1.2),
        Vec3::new(0.7, 0.8, -1.2),
        Vec3::new(0.54, 1.5, -0.95),
        Vec3::new(-0.54, 1.5, -0.95),
        [0.24, 0.65, 0.63],
    );
    let hull_mesh = meshes.add(hull.smooth().mesh());
    let metal_mesh = meshes.add(metal.mesh());
    let sail_original = fabric.positions.clone();
    let sail_mesh = meshes.add(fabric.mesh());
    let glass_mesh = meshes.add(glass.mesh());
    let hull_material = materials.add(StandardMaterial {
        perceptual_roughness: 0.53,
        metallic: 0.15,
        ..default()
    });
    let metal_material = materials.add(StandardMaterial {
        metallic: 0.72,
        perceptual_roughness: 0.31,
        ..default()
    });
    let sail_material = materials.add(StandardMaterial {
        cull_mode: None,
        double_sided: true,
        perceptual_roughness: 0.94,
        ..default()
    });
    let glass_material = materials.add(StandardMaterial {
        metallic: 0.6,
        perceptual_roughness: 0.07,
        cull_mode: None,
        ..default()
    });
    let engine_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.5, 0.6),
        emissive: LinearRgba::rgb(0.1, 3.0, 3.8),
        ..default()
    });
    let shield_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.8, 1.0, 0.1),
        emissive: LinearRgba::rgb(0.03, 0.25, 0.35),
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        unlit: true,
        ..default()
    });
    let sphere = meshes.add(Sphere::new(1.0).mesh().uv(16, 10));
    let mut rotor = Geometry::default();
    rotor.cuboid(
        Vec3::ZERO,
        Vec3::new(1.35, 0.11, 0.1),
        Quat::IDENTITY,
        bronze,
    );
    rotor.cuboid(
        Vec3::ZERO,
        Vec3::new(0.11, 1.35, 0.1),
        Quat::IDENTITY,
        bronze,
    );
    let rotor_mesh = meshes.add(rotor.mesh());
    commands
        .spawn((
            Ship,
            Transform::from_translation(crate::level::START),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((Mesh3d(hull_mesh), MeshMaterial3d(hull_material)));
            parent.spawn((Mesh3d(metal_mesh), MeshMaterial3d(metal_material.clone())));
            parent.spawn((
                Mesh3d(sail_mesh.clone()),
                MeshMaterial3d(sail_material),
                Sail {
                    mesh: sail_mesh,
                    original: sail_original,
                },
            ));
            parent.spawn((Mesh3d(glass_mesh), MeshMaterial3d(glass_material)));
            for side in [-1.0, 1.0] {
                parent.spawn((
                    Propeller,
                    Mesh3d(rotor_mesh.clone()),
                    MeshMaterial3d(metal_material.clone()),
                    Transform::from_xyz(side * 2.4, 0.0, 2.3),
                ));
                parent.spawn((
                    Exhaust,
                    Mesh3d(sphere.clone()),
                    MeshMaterial3d(engine_material.clone()),
                    Transform::from_xyz(side * 2.4, 0.0, 2.9).with_scale(Vec3::new(0.2, 0.2, 0.9)),
                    bevy::light::NotShadowCaster,
                ));
            }
            parent.spawn((
                Shield,
                Mesh3d(sphere),
                MeshMaterial3d(shield_material),
                Transform::from_scale(Vec3::new(6.7, 3.4, 5.0)),
                Visibility::Hidden,
                bevy::light::NotShadowCaster,
            ));
        });
}

pub fn animate(
    time: Res<Time>,
    pilot: Res<Pilot>,
    mut ship: Query<&mut Transform, With<Ship>>,
    mut rotors: Query<&mut Transform, (With<Propeller>, Without<Ship>, Without<Exhaust>)>,
    mut exhaust: Query<&mut Transform, (With<Exhaust>, Without<Ship>, Without<Propeller>)>,
    mut shields: Query<&mut Visibility, With<Shield>>,
    sails: Query<&Sail>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for mut transform in &mut ship {
        transform.translation = pilot.body.position;
        transform.rotation = pilot.body.rotation();
    }
    for mut transform in &mut rotors {
        transform.rotate_local_z(time.delta_secs() * (34.0 + pilot.body.velocity.length() * 2.0));
    }
    for mut transform in &mut exhaust {
        transform.scale.z = if pilot.body.boosting {
            2.5
        } else {
            0.6 + pilot.controls.throttle * 0.4
        };
    }
    for mut visible in &mut shields {
        *visible = if pilot.body.shielding {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    for sail in &sails {
        if let Some(mut mesh) = meshes.get_mut(&sail.mesh) {
            let positions: Vec<[f32; 3]> = sail
                .original
                .iter()
                .map(|p| {
                    let mut v = *p;
                    v[1] += (time.elapsed_secs() * 5.4 + p[0] * 2.4 + p[2] * 3.5).sin()
                        * 0.045
                        * (p[0].abs() / 5.0).min(1.0);
                    v
                })
                .collect();
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        }
    }
}
