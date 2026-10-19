pub mod combat;
mod input;
pub mod mission;
pub mod simulation;

use crate::flight::{Controls, FlightBody};
use bevy::prelude::*;
use combat::Threats;
use mission::Mission;

#[derive(Resource, Default)]
pub struct Pilot {
    pub body: FlightBody,
    pub controls: Controls,
    pub impact_cooldown: f32,
    pub reel: f32,
    pub fire: bool,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Pilot>()
            .init_resource::<Mission>()
            .init_resource::<Threats>()
            .insert_resource(Time::<Fixed>::from_hz(120.0))
            .add_systems(
                Update,
                input::read_controls.in_set(crate::app::FrameSet::Input),
            )
            .add_systems(FixedUpdate, simulation::step);
    }
}
