use bevy::prelude::*;

pub const SHIP_MASS: f32 = 320.0;

#[derive(Clone, Copy, Default, Debug)]
pub struct Controls {
    pub throttle: f32,
    pub turn: f32,
    pub climb: f32,
    pub brake: bool,
    pub boost: bool,
    pub shield: bool,
}

#[derive(Clone, Debug)]
pub struct FlightBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub bank: f32,
    pub yaw_rate: f32,
    pub energy: f32,
    pub heat: f32,
    pub hull: f32,
    pub boosting: bool,
    pub shielding: bool,
}

impl Default for FlightBody {
    fn default() -> Self {
        Self {
            position: crate::level::START,
            velocity: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
            bank: 0.0,
            yaw_rate: 0.0,
            energy: 100.0,
            heat: 0.0,
            hull: 100.0,
            boosting: false,
            shielding: false,
        }
    }
}

impl FlightBody {
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            -self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            -self.yaw.cos() * self.pitch.cos(),
        )
    }

    pub fn rotation(&self) -> Quat {
        Quat::from_rotation_y(self.yaw)
            * Quat::from_rotation_x(self.pitch)
            * Quat::from_rotation_z(self.bank)
    }

    pub fn step(&mut self, controls: Controls, wind: Vec3, cargo_mass: f32, dt: f32) {
        self.boosting = controls.boost && self.energy > 2.0 && self.heat < 98.0;
        self.shielding = controls.shield && self.energy > 1.0;
        let steering = (1.0 - self.velocity.length() / 110.0).clamp(0.55, 1.0);
        let target_rate = controls.turn * 0.95 * steering;
        self.yaw_rate += (target_rate - self.yaw_rate) * (1.0 - (-3.4 * dt).exp());
        self.yaw += self.yaw_rate * dt;
        self.pitch += (controls.climb * 0.28 - self.pitch) * (1.0 - (-2.5 * dt).exp());
        self.bank += (self.yaw_rate * 0.7 - self.bank) * (1.0 - (-3.8 * dt).exp());

        let forward = self.forward();
        let right = Vec3::new(self.yaw.cos(), 0.0, -self.yaw.sin());
        let air = self.velocity - wind;
        let forward_speed = air.dot(forward);
        let side_speed = air.dot(right);
        let thrust =
            controls.throttle.clamp(0.0, 1.0) * if self.boosting { 6_800.0 } else { 3_100.0 };
        let engine = forward * thrust;
        let drag = -air * air.length() * if controls.brake { 14.0 } else { 3.9 };
        let keel = -right * side_speed * 260.0;
        let wing_lift = (forward_speed.max(0.0).powi(2) * 0.55).min(480.0);
        let float_lift = SHIP_MASS * 9.81 * 0.94 + cargo_mass * 9.81;
        let climb = controls.climb * 1_850.0 / (1.0 + cargo_mass / SHIP_MASS);
        let vertical =
            Vec3::Y * (float_lift + wing_lift + climb - SHIP_MASS * 9.81 - self.velocity.y * 310.0);
        self.velocity += (engine + drag + keel + vertical) / SHIP_MASS * dt;
        self.position += self.velocity * dt;

        let spend =
            if self.boosting { 20.0 } else { 0.0 } + if self.shielding { 14.0 } else { 0.0 };
        let recovery = if spend > 0.0 { 0.0 } else { 7.5 };
        self.energy = (self.energy + (recovery - spend) * dt).clamp(0.0, 100.0);
        let warming = if self.boosting { 24.0 } else { -12.0 };
        self.heat = (self.heat + warming * dt).clamp(0.0, 100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thrust_builds_speed_and_drag_limits_it() {
        let mut ship = FlightBody::default();
        let input = Controls {
            throttle: 1.0,
            ..default()
        };
        for _ in 0..3600 {
            ship.step(input, Vec3::ZERO, 0.0, 1.0 / 120.0);
        }
        assert!((-ship.velocity.z - 28.0).abs() < 2.0);
        assert!(ship.velocity.x.abs() < 0.01);
    }

    #[test]
    fn braking_reduces_speed_without_reversing() {
        let mut ship = FlightBody {
            velocity: -Vec3::Z * 28.0,
            ..default()
        };
        for _ in 0..360 {
            ship.step(
                Controls {
                    brake: true,
                    ..default()
                },
                Vec3::ZERO,
                0.0,
                1.0 / 120.0,
            );
        }
        assert!(ship.velocity.z < 0.0 && ship.velocity.length() < 7.0);
    }

    #[test]
    fn banking_follows_turn_and_relaxes() {
        let mut ship = FlightBody::default();
        for _ in 0..240 {
            ship.step(
                Controls {
                    turn: 1.0,
                    ..default()
                },
                Vec3::ZERO,
                0.0,
                1.0 / 120.0,
            );
        }
        assert!(ship.yaw > 1.4 && ship.bank > 0.5);
        for _ in 0..480 {
            ship.step(Controls::default(), Vec3::ZERO, 0.0, 1.0 / 120.0);
        }
        assert!(ship.bank.abs() < 0.01);
    }

    #[test]
    fn left_and_right_turns_lower_the_inside_wing() {
        for turn in [-1.0, 1.0] {
            let mut ship = FlightBody {
                velocity: -Vec3::Z * 20.0,
                ..default()
            };
            for _ in 0..120 {
                ship.step(
                    Controls {
                        throttle: 0.8,
                        turn,
                        ..default()
                    },
                    Vec3::ZERO,
                    0.0,
                    1.0 / 120.0,
                );
            }
            assert!(ship.position.x * turn < -1.0);
            assert!((ship.rotation() * Vec3::X).y * turn > 0.2);
            assert!(ship.forward().dot(ship.rotation() * -Vec3::Z) > 0.999);
        }
    }

    #[test]
    fn boost_and_shield_share_finite_energy() {
        let mut ship = FlightBody::default();
        for _ in 0..600 {
            ship.step(
                Controls {
                    throttle: 1.0,
                    boost: true,
                    shield: true,
                    ..default()
                },
                Vec3::ZERO,
                0.0,
                1.0 / 120.0,
            );
        }
        assert!(ship.energy < 3.0);
        assert!(ship.energy >= 0.0);
        assert!(ship.heat <= 100.0);
    }
}
