use bevy::prelude::*;
use storm_edge::{
    flight::Controls,
    game::{
        Pilot,
        combat::Threats,
        mission::{Mission, Phase},
        simulation::advance,
    },
    level::ISLANDS,
};

#[test]
fn expedition_can_be_flown_with_a_towed_payload() {
    let mut pilot = Pilot::default();
    let mut mission = Mission {
        phase: Phase::Flying,
        ..default()
    };
    let mut threats = Threats::default();
    threats.hunters.clear();
    let mut attached_frames = 0;
    for _ in 0..120 * 600 {
        let goal = mission.target();
        let body = &pilot.body;
        let delta = goal - body.position;
        let horizontal = Vec2::new(delta.x, delta.z).length();
        let desired = (-delta.x).atan2(-delta.z);
        let error = (desired - body.yaw + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU)
            - std::f32::consts::PI;
        let mut altitude = goal.y;
        for island in ISLANDS {
            let offset = body.position - island.center;
            if Vec2::new(offset.x, offset.z).length() < island.radius + 20.0 {
                altitude = altitude.max(
                    island.surface(body.position.x, body.position.z)
                        + 14.0
                        + if mission.payload.is_some() { 8.0 } else { 0.0 },
                );
            }
        }
        pilot.controls = Controls {
            throttle: if horizontal < 40.0 || error.abs() > 0.8 {
                0.16
            } else {
                0.8
            },
            turn: (error * 1.7 - body.yaw_rate * 0.55).clamp(-1.0, 1.0),
            climb: ((altitude - body.position.y) * 0.65 - body.velocity.y * 0.8).clamp(-6.0, 6.0)
                / 6.0,
            brake: horizontal < 30.0,
            ..default()
        };
        if mission.payload.is_some() {
            attached_frames += 1;
        }
        if delta.length() < 18.0 && body.velocity.length() < 17.0 {
            mission.interact(&mut pilot.body);
        }
        advance(1.0 / 120.0, &mut pilot, &mut mission, &mut threats);
        if mission.phase == Phase::Won {
            break;
        }
        assert_ne!(mission.phase, Phase::Lost, "{:?}", pilot.body);
    }
    assert_eq!(
        mission.phase,
        Phase::Won,
        "delivered={} body={:?}",
        mission.delivered,
        pilot.body
    );
    assert!(attached_frames > 1200);
    assert!(mission.elapsed > 60.0);
}

#[test]
fn pause_freezes_ship_cargo_projectiles_and_storm() {
    let mut pilot = Pilot::default();
    let mut mission = Mission {
        phase: Phase::Paused,
        ..default()
    };
    let mut threats = Threats::default();
    pilot.controls = Controls {
        throttle: 1.0,
        boost: true,
        climb: 1.0,
        ..default()
    };
    let position = pilot.body.position;
    let hunter = threats.hunters[0].position;
    for _ in 0..1200 {
        advance(1.0 / 120.0, &mut pilot, &mut mission, &mut threats);
    }
    assert_eq!(pilot.body.position, position);
    assert_eq!(threats.hunters[0].position, hunter);
    assert_eq!(mission.elapsed, 0.0);
    assert_eq!(pilot.body.energy, 100.0);
}
