use super::mission::Mission;
use crate::{flight::FlightBody, level::ISLANDS};
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct Hunter {
    pub position: Vec3,
    pub velocity: Vec3,
    pub health: f32,
    pub cooldown: f32,
    pub alert: f32,
    pub home: Vec3,
}

#[derive(Clone, Debug)]
pub struct Pulse {
    pub position: Vec3,
    pub previous: Vec3,
    pub velocity: Vec3,
    pub ttl: f32,
    pub hostile: bool,
}

#[derive(Resource, Debug)]
pub struct Threats {
    pub hunters: Vec<Hunter>,
    pub pulses: Vec<Pulse>,
    pub downed: u32,
}

impl Default for Threats {
    fn default() -> Self {
        Self {
            hunters: [
                Vec3::new(150.0, 85.0, -180.0),
                Vec3::new(-70.0, 120.0, -270.0),
                Vec3::new(40.0, 135.0, -390.0),
            ]
            .into_iter()
            .map(|p| Hunter {
                position: p,
                home: p,
                velocity: Vec3::ZERO,
                health: 2.0,
                cooldown: 3.0,
                alert: 0.0,
            })
            .collect(),
            pulses: Vec::new(),
            downed: 0,
        }
    }
}

fn visible(a: Vec3, b: Vec3) -> bool {
    (1..12).all(|step| {
        let point = a.lerp(b, step as f32 / 12.0);
        ISLANDS
            .iter()
            .all(|island| island.penetration(point, 1.0).is_none())
    })
}

impl Threats {
    pub fn shoot(&mut self, ship: &mut FlightBody, mission: &mut Mission) {
        if mission.shot_cooldown > 0.0 || ship.energy < 12.0 {
            return;
        }
        let position = ship.position + ship.forward() * 4.0 - Vec3::Y * 0.25;
        self.pulses.push(Pulse {
            position,
            previous: position,
            velocity: ship.forward() * 120.0 + ship.velocity,
            ttl: 1.7,
            hostile: false,
        });
        ship.energy -= 12.0;
        mission.shots += 1;
        mission.shot_cooldown = 0.36;
    }

    pub fn step(&mut self, ship: &mut FlightBody, mission: &mut Mission, dt: f32) {
        let active =
            mission.delivered >= 1 || mission.payload.as_ref().is_some_and(|p| p.relic >= 1);
        for (index, hunter) in self.hunters.iter_mut().enumerate() {
            if hunter.health <= 0.0 {
                hunter.velocity.y -= dt * 5.0;
                hunter.position += hunter.velocity * dt;
                continue;
            }
            let delta = ship.position - hunter.position;
            let distance = delta.length();
            let can_see = active
                && distance < 100.0 + ship.heat * 1.2
                && visible(hunter.position, ship.position);
            hunter.alert =
                (hunter.alert + if can_see { dt * 0.7 } else { -dt * 0.25 }).clamp(0.0, 1.0);
            let patrol = hunter.home
                + Vec3::new(
                    (mission.elapsed * 0.16 + index as f32).cos() * 24.0,
                    (mission.elapsed * 0.35).sin() * 5.0,
                    (mission.elapsed * 0.16 + index as f32).sin() * 24.0,
                );
            let target = if hunter.alert > 0.2 {
                ship.position + ship.velocity * 0.4
            } else {
                patrol
            };
            let desired = (target - hunter.position).normalize_or_zero()
                * if hunter.alert > 0.2 { 21.0 } else { 10.0 };
            hunter.velocity = hunter.velocity.lerp(desired, 1.0 - (-1.6 * dt).exp());
            hunter.position += hunter.velocity * dt;
            for island in ISLANDS {
                if let Some(push) = island.penetration(hunter.position, 5.0) {
                    hunter.position += push;
                    hunter.velocity += push.normalize_or_zero() * dt * 40.0;
                }
            }
            hunter.cooldown -= dt;
            if hunter.alert > 0.55 && hunter.cooldown <= 0.0 && distance < 85.0 && can_see {
                let position = hunter.position;
                let velocity = (ship.position + ship.velocity * distance / 65.0 - position)
                    .normalize_or_zero()
                    * 65.0;
                self.pulses.push(Pulse {
                    position,
                    previous: position,
                    velocity,
                    ttl: 2.5,
                    hostile: true,
                });
                hunter.cooldown = 2.8;
            }
        }
        for pulse in &mut self.pulses {
            pulse.previous = pulse.position;
            pulse.position += pulse.velocity * dt;
            pulse.ttl -= dt;
            if pulse.hostile && pulse.position.distance(ship.position) < 3.2 {
                mission.damage(ship, 14.0);
                pulse.ttl = 0.0;
            }
            if !pulse.hostile {
                for hunter in &mut self.hunters {
                    if hunter.health > 0.0 && pulse.position.distance(hunter.position) < 5.0 {
                        hunter.health -= 1.0;
                        hunter.velocity += pulse.velocity.normalize_or_zero() * 8.0;
                        pulse.ttl = 0.0;
                        if hunter.health <= 0.0 {
                            self.downed += 1;
                            mission.announce("Страж потерял тягу. Проход свободен.");
                        }
                        break;
                    }
                }
            }
            if ISLANDS
                .iter()
                .any(|island| island.penetration(pulse.position, 0.0).is_some())
            {
                pulse.ttl = 0.0;
            }
        }
        self.pulses.retain(|p| p.ttl > 0.0);
    }
}
