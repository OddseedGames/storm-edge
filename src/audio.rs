use crate::game::{
    Pilot,
    mission::{Mission, Phase},
};
use bevy::{audio::Volume, prelude::*};

pub fn update(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mission: Res<Mission>,
    pilot: Res<Pilot>,
    mut started: Local<bool>,
    mut sinks: Query<&mut AudioSink>,
) {
    if mission.phase == Phase::Flying && !*started {
        commands.spawn((
            AudioPlayer::new(assets.load("audio/wind.wav")),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(0.2)),
        ));
        *started = true;
    }
    for mut sink in &mut sinks {
        let volume = if mission.muted {
            0.0
        } else if mission.phase == Phase::Flying {
            0.17 + pilot.body.velocity.length() * 0.004
        } else {
            0.045
        };
        sink.set_volume(Volume::Linear(volume));
        sink.set_speed(0.8 + pilot.body.velocity.length() * 0.009);
    }
}
