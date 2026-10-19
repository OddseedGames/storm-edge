use crate::{game::mission::Mission, level::ISLANDS};
use bevy::{
    light::{FogVolume, VolumetricLight},
    mesh::MeshVertexBufferLayoutRef,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct AtmosphereMaterial {
    #[uniform(0)]
    pub parameters: Vec4,
}

impl Material for AtmosphereMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/atmosphere.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        if self.parameters.w > 1.5 {
            AlphaMode::Blend
        } else {
            AlphaMode::Opaque
        }
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

#[derive(Component)]
pub struct Sky;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<AtmosphereMaterial>>,
) {
    commands.spawn((
        Sky,
        Mesh3d(meshes.add(Sphere::new(2200.0).mesh().uv(32, 16))),
        MeshMaterial3d(materials.add(AtmosphereMaterial {
            parameters: Vec4::ZERO,
        })),
        Transform::default(),
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
    ));
    let waterfall = materials.add(AtmosphereMaterial {
        parameters: Vec4::new(0.0, 0.0, 0.0, 2.0),
    });
    for (i, island) in ISLANDS.iter().enumerate().take(4) {
        for side in [-1.0, 1.0] {
            let x = island.center.x + side * island.radius * 0.89;
            let z = island.center.z + island.radius * 0.15;
            let top = island.surface(x, z);
            let length = top + 65.0;
            commands.spawn((
                Mesh3d(meshes.add(Rectangle::new(3.0 + i as f32 * 1.4, length))),
                MeshMaterial3d(waterfall.clone()),
                Transform::from_xyz(x, top - length * 0.5, z)
                    .with_rotation(Quat::from_rotation_y(side * 0.8)),
                bevy::light::NotShadowCaster,
            ));
        }
    }
    commands.spawn((
        FogVolume {
            density_factor: 0.00008,
            fog_color: Color::srgb(0.72, 0.85, 0.83),
            scattering_asymmetry: 0.6,
            ..default()
        },
        Transform::from_scale(Vec3::splat(5000.0)),
    ));
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.83, 0.61),
            illuminance: 14_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-180.0, 230.0, -240.0).looking_at(Vec3::ZERO, Vec3::Y),
        bevy::light::CascadeShadowConfigBuilder {
            maximum_distance: 330.0,
            first_cascade_far_bound: 38.0,
            num_cascades: 3,
            ..default()
        }
        .build(),
        VolumetricLight,
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.65, 0.83, 0.9),
        brightness: 320.0,
        ..default()
    });
}

pub fn animate(
    time: Res<Time>,
    mission: Res<Mission>,
    mut materials: ResMut<Assets<AtmosphereMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.parameters.x = time.elapsed_secs();
        material.parameters.y = mission.storm;
    }
}
