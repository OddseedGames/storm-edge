mod controls;
mod hud;
mod layout;
mod menus;

use crate::{
    game::{
        Pilot,
        combat::Threats,
        mission::{Mission, Phase},
    },
    graphics::{
        camera::{FlightCamera, SceneTarget, ViewSettings},
        effects::Trails,
    },
    level::RELICS,
};
use bevy::prelude::*;

#[derive(Component)]
pub(super) enum Item {
    Root,
    Intro,
    Hud,
    Overlay,
    OverlayTitle,
    OverlayBody,
    OverlayButton,
    Stats,
    Objective,
    Compass,
    Notice,
    Threat,
    Target,
    Crosshair,
    Bar(u8),
}
#[derive(Component)]
pub(super) enum Action {
    Start,
    Continue,
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, layout::setup).add_systems(
            Update,
            (controls::buttons, hud::update)
                .chain()
                .in_set(crate::app::FrameSet::Ui),
        );
    }
}

const CREAM: Color = Color::srgb(0.94, 0.9, 0.77);
const MUTED: Color = Color::srgb(0.62, 0.75, 0.73);
const GOLD: Color = Color::srgb(0.94, 0.7, 0.31);

fn text(value: impl Into<String>, font: &Handle<Font>, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(value),
        TextFont {
            font: font.clone().into(),
            font_size: size.into(),
            ..default()
        },
        TextColor(color),
    )
}
