use super::dynamics::{FlightBody, SHIP_MASS};
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct Payload {
    pub relic: usize,
    pub position: Vec3,
    pub velocity: Vec3,
    pub length: f32,
    pub mass: f32,
    pub tension: f32,
}

impl Payload {
    pub fn attach(relic: usize, ship: &FlightBody, mass: f32) -> Self {
        Self {
            relic,
            position: ship.position - Vec3::Y * 8.0,
            velocity: ship.velocity,
            length: 6.0,
            mass,
            tension: 0.0,
        }
    }

    pub fn step(&mut self, ship: &mut FlightBody, wind: Vec3, dt: f32) {
        self.velocity += (-Vec3::Y * 9.81 - (self.velocity - wind) * 0.17) * dt;
        self.position += self.velocity * dt;
        let anchor = ship.position - Vec3::Y * 2.0;
        let offset = self.position - anchor;
        let distance = offset.length();
        self.tension = 0.0;
        if distance > self.length {
            let direction = offset / distance;
            let inv_mass = 1.0 / SHIP_MASS + 1.0 / self.mass;
            let correction = (distance - self.length) / inv_mass;
            ship.position += direction * correction / SHIP_MASS;
            self.position -= direction * correction / self.mass;
            let separating = (self.velocity - ship.velocity).dot(direction);
            if separating > 0.0 {
                let impulse = separating / inv_mass;
                ship.velocity += direction * impulse / SHIP_MASS;
                self.velocity -= direction * impulse / self.mass;
                self.tension = impulse / dt;
            }
        }
    }

    pub fn reel(&mut self, direction: f32, dt: f32) {
        self.length = (self.length + direction * 3.0 * dt).clamp(3.5, 12.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flight::Controls;

    #[test]
    fn cable_stays_taut_during_acceleration_and_banking() {
        let mut ship = FlightBody::default();
        let mut cargo = Payload::attach(0, &ship, 105.0);
        for frame in 0..2400 {
            let controls = Controls {
                throttle: 0.8,
                turn: if frame > 1200 { 0.8 } else { 0.0 },
                ..default()
            };
            ship.step(controls, Vec3::ZERO, cargo.mass, 1.0 / 120.0);
            cargo.step(&mut ship, Vec3::ZERO, 1.0 / 120.0);
            let distance = cargo.position.distance(ship.position - Vec3::Y * 2.0);
            assert!(distance <= cargo.length + 0.001);
            assert!(ship.position.is_finite() && cargo.velocity.is_finite());
        }
    }

    #[test]
    fn constraint_conserves_linear_momentum() {
        let mut ship = FlightBody::default();
        let mut cargo = Payload::attach(0, &ship, 120.0);
        cargo.position.x += 6.0;
        cargo.velocity = Vec3::X * 10.0;
        let before = ship.velocity * SHIP_MASS + cargo.velocity * cargo.mass;
        cargo.step(&mut ship, Vec3::ZERO, 1.0 / 120.0);
        let expected = before + (-Vec3::Y * 9.81 - Vec3::X * 1.7) * cargo.mass / 120.0;
        let after = ship.velocity * SHIP_MASS + cargo.velocity * cargo.mass;
        assert!(after.distance(expected) < 0.001);
        assert!(ship.velocity.x > 0.0);
    }

    #[test]
    fn reel_limits_are_physical_lengths() {
        let ship = FlightBody::default();
        let mut cargo = Payload::attach(0, &ship, 70.0);
        cargo.reel(-1.0, 10.0);
        assert_eq!(cargo.length, 3.5);
        cargo.reel(1.0, 10.0);
        assert_eq!(cargo.length, 12.0);
    }
}
