pub mod architecture;
pub mod atmosphere;
pub mod camera;
pub mod effects;
pub mod geometry;
pub mod ship;
pub mod surface;
pub mod terrain;
pub mod world;

use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<atmosphere::AtmosphereMaterial>::default())
            .add_plugins(MaterialPlugin::<surface::IslandMaterial>::default())
            .init_resource::<camera::ViewSettings>()
            .add_systems(PreStartup, camera::setup)
            .add_systems(
                Startup,
                (world::setup, atmosphere::setup, ship::spawn, effects::setup),
            )
            .add_systems(
                Update,
                (
                    ship::animate,
                    atmosphere::animate,
                    world::animate,
                    effects::animate,
                    camera::follow,
                    surface::prepare_mips,
                )
                    .chain()
                    .in_set(crate::app::FrameSet::Graphics),
            );
    }
}
