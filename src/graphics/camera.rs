use crate::game::{
    Pilot,
    mission::{Mission, Phase},
};
use bevy::{
    camera::RenderTarget,
    core_pipeline::tonemapping::Tonemapping,
    light::{EnvironmentMapLight, VolumetricFog},
    post_process::bloom::Bloom,
    prelude::*,
    render::render_resource::TextureFormat,
};

#[derive(Component)]
pub struct FlightCamera;

#[derive(Resource)]
pub struct SceneTarget {
    pub image: Handle<Image>,
    pub size: UVec2,
    pub ui_camera: Entity,
}

#[derive(Resource, Default)]
pub struct ViewSettings {
    pub photo: bool,
    pub low: bool,
}

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, windows: Query<&Window>) {
    let window = windows.single().expect("primary window");
    let width = window.width().min(1600.0).max(640.0) as u32;
    let height = (width as f32 * window.height() / window.width()).max(360.0) as u32;
    let image = images.add(Image::new_target_texture(
        width,
        height,
        TextureFormat::Rgba8UnormSrgb,
        None,
    ));
    let ui_camera = commands
        .spawn((
            Camera2d,
            Camera {
                order: 0,
                ..default()
            },
        ))
        .id();
    commands.insert_resource(SceneTarget {
        image: image.clone(),
        size: UVec2::new(width, height),
        ui_camera,
    });
    let mut environment = EnvironmentMapLight::hemispherical_gradient(
        &mut images,
        Color::srgb(0.54, 0.74, 0.86),
        Color::srgb(0.77, 0.76, 0.6),
        Color::srgb(0.22, 0.3, 0.27),
    );
    environment.intensity = 420.0;
    commands.spawn((
        FlightCamera,
        Camera3d::default(),
        Camera {
            order: -1,
            ..default()
        },
        RenderTarget::Image(image.into()),
        Transform::from_xyz(18.0, 53.0, 123.0).looking_at(Vec3::new(-14.0, 47.0, 42.0), Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            fov: 58.0f32.to_radians(),
            near: 0.2,
            far: 4000.0,
            ..default()
        }),
        Tonemapping::AcesFitted,
        Msaa::Sample4,
        Bloom {
            intensity: 0.13,
            ..Bloom::NATURAL
        },
        DistanceFog {
            color: Color::srgb(0.64, 0.75, 0.72),
            directional_light_color: Color::srgba(1.0, 0.8, 0.54, 0.4),
            directional_light_exponent: 12.0,
            falloff: FogFalloff::ExponentialSquared { density: 0.0011 },
        },
        VolumetricFog {
            step_count: 32,
            ambient_intensity: 0.8,
            ambient_color: Color::srgb(0.65, 0.77, 0.81),
            ..default()
        },
        environment,
    ));
}

pub fn follow(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    pilot: Res<Pilot>,
    mut mission: ResMut<Mission>,
    mut settings: ResMut<ViewSettings>,
    mut camera: Query<(Entity, &mut Transform, &mut Projection), With<FlightCamera>>,
    mut lights: Query<&mut DirectionalLight>,
    windows: Query<&Window>,
    mut target: ResMut<SceneTarget>,
    mut images: ResMut<Assets<Image>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        settings.photo = !settings.photo;
    }
    if keys.just_pressed(KeyCode::F2) {
        settings.low = !settings.low;
        for (entity, _, _) in &mut camera {
            if settings.low {
                commands.entity(entity).remove::<VolumetricFog>();
            } else {
                commands.entity(entity).insert(VolumetricFog {
                    step_count: 32,
                    ..default()
                });
            }
        }
        for mut light in &mut lights {
            light.shadow_maps_enabled = !settings.low;
        }
        mission.announce(if settings.low {
            "Графика: экономичный режим"
        } else {
            "Графика: полное качество"
        });
    }
    if let Ok(window) = windows.single() {
        let width = window
            .width()
            .min(if settings.low { 1024.0 } else { 1600.0 })
            .max(640.0) as u32;
        let height = (width as f32 * window.height() / window.width()).max(360.0) as u32;
        let size = UVec2::new(width, height);
        if size != target.size {
            if let Some(mut image) = images.get_mut(&target.image) {
                image.resize(bevy::render::render_resource::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                });
            }
            target.size = size;
        }
    }
    for (_, mut transform, mut projection) in &mut camera {
        let yaw = Quat::from_rotation_y(pilot.body.yaw);
        let speed = pilot.body.velocity.length();
        let (position, look_at) = if mission.phase == Phase::Intro {
            let drift = (time.elapsed_secs() * 0.06).sin() * 4.0;
            (
                pilot.body.position + Vec3::new(18.0 + drift, 11.0, 25.0),
                pilot.body.position + Vec3::new(-14.0, 5.0, -56.0),
            )
        } else {
            let back = if pilot.body.boosting { 23.0 } else { 20.0 };
            let position =
                pilot.body.position + yaw * Vec3::new(0.0, 7.0, back) - pilot.body.velocity * 0.045;
            (
                position,
                pilot.body.position + yaw * Vec3::new(0.0, 2.5, -13.0) + pilot.body.velocity * 0.1,
            )
        };
        let ease = 1.0 - (-time.delta_secs().min(0.1) * 5.0).exp();
        transform.translation = transform.translation.lerp(position, ease);
        let goal = Transform::from_translation(transform.translation).looking_at(look_at, Vec3::Y);
        transform.rotation = transform.rotation.slerp(goal.rotation, ease);
        if let Projection::Perspective(perspective) = projection.as_mut() {
            let target = (58.0 + (speed / 45.0).clamp(0.0, 1.0) * 7.0).to_radians();
            perspective.fov += (target - perspective.fov) * ease;
        }
    }
}
