use super::{
    Pilot,
    combat::Threats,
    mission::{Mission, Phase},
};
use bevy::prelude::*;

pub fn read_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut pilot: ResMut<Pilot>,
    mut mission: ResMut<Mission>,
    mut threats: ResMut<Threats>,
    mut trails: ResMut<crate::graphics::effects::Trails>,
    windows: Query<&Window>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        match mission.phase {
            Phase::Intro => {
                mission.phase = Phase::Flying;
                mission.announce(
                    "Первый маяк — слева по курсу. A / D — поворот, пробел / C — высота.",
                );
            }
            Phase::Paused => {
                mission.phase = Phase::Flying;
                mission.show_help = false;
            }
            Phase::Won | Phase::Lost => {
                let muted = mission.muted;
                *mission = Mission::default();
                mission.muted = muted;
                *pilot = Pilot::default();
                *threats = Threats::default();
                *trails = default();
            }
            _ => {}
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        mission.phase = match mission.phase {
            Phase::Flying => Phase::Paused,
            Phase::Paused => Phase::Flying,
            other => other,
        };
        mission.show_help = false;
    }
    if keys.just_pressed(KeyCode::KeyH) && matches!(mission.phase, Phase::Flying | Phase::Paused) {
        mission.show_help = !mission.show_help;
        mission.phase = if mission.show_help {
            Phase::Paused
        } else {
            Phase::Flying
        };
    }
    if keys.just_pressed(KeyCode::KeyM) {
        mission.muted = !mission.muted;
    }
    if keys.just_pressed(KeyCode::KeyR)
        && matches!(mission.phase, Phase::Paused | Phase::Won | Phase::Lost)
    {
        let muted = mission.muted;
        *mission = Mission::default();
        mission.muted = muted;
        *pilot = Pilot::default();
        *threats = Threats::default();
        *trails = default();
    }
    if mission.phase == Phase::Flying && windows.iter().any(|w| !w.focused) {
        mission.phase = Phase::Paused;
    }
    if mission.phase != Phase::Flying {
        pilot.controls = default();
        pilot.fire = false;
        return;
    }
    let axis = |positive: &[KeyCode], negative: &[KeyCode]| {
        let p = positive.iter().any(|k| keys.pressed(*k)) as i32;
        let n = negative.iter().any(|k| keys.pressed(*k)) as i32;
        (p - n) as f32
    };
    let brake = keys.pressed(KeyCode::KeyS);
    pilot.controls = crate::flight::Controls {
        throttle: if brake {
            0.0
        } else if keys.pressed(KeyCode::KeyW) {
            1.0
        } else {
            0.5
        },
        turn: axis(
            &[KeyCode::KeyA, KeyCode::ArrowLeft],
            &[KeyCode::KeyD, KeyCode::ArrowRight],
        ),
        climb: axis(
            &[KeyCode::Space, KeyCode::ArrowUp],
            &[KeyCode::KeyC, KeyCode::ArrowDown],
        ),
        brake,
        boost: keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight),
        shield: keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight),
    };
    pilot.reel = axis(&[KeyCode::KeyZ], &[KeyCode::KeyQ]);
    pilot.fire = mouse.pressed(MouseButton::Left) || keys.pressed(KeyCode::KeyF);
    if keys.just_pressed(KeyCode::KeyE) {
        mission.interact(&mut pilot.body);
    }
    if keys.just_pressed(KeyCode::KeyX) {
        mission.release(&pilot.body);
    }
}
