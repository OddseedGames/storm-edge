use super::{
    Pilot,
    combat::Threats,
    mission::{Mission, Phase},
};
use crate::{
    flight::wind,
    level::{HOME, ISLANDS, WORLD_RADIUS},
};
use bevy::prelude::*;

pub fn step(
    time: Res<Time<Fixed>>,
    mut pilot: ResMut<Pilot>,
    mut mission: ResMut<Mission>,
    mut threats: ResMut<Threats>,
) {
    advance(time.delta_secs(), &mut pilot, &mut mission, &mut threats);
}

pub fn advance(dt: f32, pilot: &mut Pilot, mission: &mut Mission, threats: &mut Threats) {
    if mission.phase != Phase::Flying {
        return;
    }
    mission.elapsed += dt;
    mission.notice_left = (mission.notice_left - dt).max(0.0);
    mission.shot_cooldown = (mission.shot_cooldown - dt).max(0.0);
    mission.storm = (mission.delivered as f32 * 0.19 + mission.elapsed / 1600.0).clamp(0.0, 1.0);
    pilot.impact_cooldown = (pilot.impact_cooldown - dt).max(0.0);
    let air = wind::sample(pilot.body.position, mission.elapsed, mission.storm);
    let mass = mission.payload.as_ref().map_or(0.0, |p| p.mass);
    let input = pilot.controls;
    pilot.body.step(input, air, mass, dt);
    if let Some(payload) = &mut mission.payload {
        payload.reel(pilot.reel, dt);
        payload.step(&mut pilot.body, air, dt);
        for island in ISLANDS {
            if let Some(push) = island.penetration(payload.position, 1.5) {
                payload.position += push;
                let normal = push.normalize_or_zero();
                let inward = payload.velocity.dot(normal).min(0.0);
                payload.velocity -= normal * inward;
            }
        }
    }
    for island in ISLANDS {
        if let Some(push) = island.penetration(pilot.body.position, 2.1) {
            pilot.body.position += push;
            let normal = push.normalize_or_zero();
            let inward = pilot.body.velocity.dot(normal).min(0.0);
            pilot.body.velocity -= normal * inward * 1.3;
            if inward < -3.0 && pilot.impact_cooldown == 0.0 {
                mission.damage(&mut pilot.body, -inward * 1.2);
                pilot.impact_cooldown = 0.8;
                mission.announce("Удар о скалу. Щит Ctrl смягчает столкновения.");
            }
        }
    }
    for solid in crate::level::colliders::architecture() {
        if let Some(push) = solid.penetration(pilot.body.position, 2.1) {
            pilot.body.position += push;
            let normal = push.normalize_or_zero();
            let inward = pilot.body.velocity.dot(normal).min(0.0);
            pilot.body.velocity -= normal * inward * 1.2;
            if inward < -3.0 && pilot.impact_cooldown == 0.0 {
                mission.damage(&mut pilot.body, -inward * 1.2);
                pilot.impact_cooldown = 0.8;
                mission.announce("Столкновение с руинами. Снизь скорость и набери высоту.");
            }
        }
        if let Some(payload) = &mut mission.payload {
            if let Some(push) = solid.penetration(payload.position, 1.5) {
                payload.position += push;
                let normal = push.normalize_or_zero();
                let inward = payload.velocity.dot(normal).min(0.0);
                payload.velocity -= normal * inward;
            }
        }
    }
    if pilot.body.position.y < -45.0 {
        mission.damage(&mut pilot.body, dt * 8.0);
        pilot.body.velocity.y += dt * 8.0;
        if mission.notice_left < 1.0 {
            mission.announce("Облачная бездна. Набирай высоту: пробел.");
        }
    }
    if pilot.body.position.y > 235.0 {
        pilot.body.velocity.y -= dt * 12.0;
    }
    if Vec2::new(pilot.body.position.x, pilot.body.position.z + 130.0).length() > WORLD_RADIUS {
        let inward = (HOME - pilot.body.position).normalize_or_zero();
        pilot.body.velocity += inward * dt * 18.0;
        if mission.notice_left < 1.0 {
            mission.announce("За пределами архипелага встречный ветер. Вернись к маяку.");
        }
    }
    if pilot.fire {
        threats.shoot(&mut pilot.body, mission);
    }
    threats.step(&mut pilot.body, mission, dt);
}
