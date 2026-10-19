use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FrameSet {
    Input,
    Graphics,
    Ui,
}

pub fn run() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Кромка бури / Storm Edge".into(),
                    resolution: if cfg!(target_arch = "wasm32") {
                        bevy::window::WindowResolution::new(1440, 900)
                    } else {
                        bevy::window::WindowResolution::new(1440, 900)
                            .with_scale_factor_override(1.0)
                    },
                    canvas: Some("#game".into()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                file_path: asset_path(),
                ..default()
            }),
    )
    .insert_resource(ClearColor(Color::srgb(0.52, 0.69, 0.71)))
    .insert_resource(bevy::light::DirectionalLightShadowMap { size: 2048 })
    .configure_sets(
        Update,
        (FrameSet::Input, FrameSet::Graphics, FrameSet::Ui).chain(),
    )
    .add_plugins((
        crate::game::GamePlugin,
        crate::graphics::GraphicsPlugin,
        crate::ui::UiPlugin,
    ))
    .add_systems(
        Update,
        (crate::audio::update, crate::browser::publish).after(FrameSet::Ui),
    );
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, capture);
    app.run();
}

fn asset_path() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id("game"))
            .and_then(|canvas| canvas.get_attribute("data-assets"))
            .unwrap_or_else(|| "assets".into())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets").into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn capture(
    mut commands: Commands,
    mut frame: Local<u32>,
    mut exit: MessageWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    use bevy::render::view::screenshot::{Screenshot, save_to_disk};
    *frame += 1;
    let path = std::env::var("STORM_CAPTURE").ok();
    if *frame == 180 {
        if let Some(path) = &path {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path.clone()));
        }
    }
    if path.is_some() && *frame == 240 {
        exit.write(AppExit::Success);
    }
    if keys.just_pressed(KeyCode::F12) {
        let _ = std::fs::create_dir_all("screenshots");
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("screenshots/storm-{}.png", *frame)));
    }
}
