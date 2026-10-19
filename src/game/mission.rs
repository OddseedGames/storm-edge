use crate::{
    flight::{FlightBody, Payload},
    level::{HOME, RELICS},
};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Phase {
    #[default]
    Intro,
    Flying,
    Paused,
    Won,
    Lost,
}

#[derive(Resource, Debug)]
pub struct Mission {
    pub phase: Phase,
    pub delivered: usize,
    pub payload: Option<Payload>,
    pub loose: [Vec3; 3],
    pub elapsed: f32,
    pub shots: u32,
    pub damage_taken: f32,
    pub notice: String,
    pub notice_left: f32,
    pub shot_cooldown: f32,
    pub storm: f32,
    pub show_help: bool,
    pub muted: bool,
}

impl Default for Mission {
    fn default() -> Self {
        Self {
            phase: Phase::Intro,
            delivered: 0,
            payload: None,
            loose: RELICS.map(|r| r.position),
            elapsed: 0.0,
            shots: 0,
            damage_taken: 0.0,
            notice: String::new(),
            notice_left: 0.0,
            shot_cooldown: 0.0,
            storm: 0.0,
            show_help: false,
            muted: false,
        }
    }
}

impl Mission {
    pub fn announce(&mut self, message: impl Into<String>) {
        self.notice = message.into();
        self.notice_left = 5.0;
    }

    pub fn target(&self) -> Vec3 {
        if self.payload.is_some() || self.delivered == 3 {
            HOME
        } else {
            self.loose[self.delivered] + Vec3::Y * 7.0
        }
    }

    pub fn interact(&mut self, ship: &mut FlightBody) {
        if self.phase != Phase::Flying {
            return;
        }
        if self.payload.is_some() {
            if ship.position.distance(HOME) < 22.0 {
                if ship.velocity.length() > 17.0 {
                    self.announce("Слишком быстро. Удерживай S, чтобы затормозить.");
                    return;
                }
                self.payload = None;
                self.delivered += 1;
                ship.hull = (ship.hull + 30.0).min(100.0);
                ship.energy = 100.0;
                if self.delivered == 3 {
                    self.phase = Phase::Won;
                    self.announce("Все три сердца в гавани. Город снова увидит рассвет.");
                } else {
                    self.announce("Груз принят. Корпус восстановлен. Новый маршрут отмечен.");
                }
            } else {
                self.announce("Груз на тросе. Вернись к светящемуся кольцу гавани.");
            }
        } else if self.delivered < 3 {
            let index = self.delivered;
            if ship.position.distance(self.target()) < 19.0 {
                if ship.velocity.length() > 20.0 {
                    self.announce("Снизь скорость: удерживай S и подойди ближе.");
                    return;
                }
                self.payload = Some(Payload::attach(index, ship, RELICS[index].mass));
                self.announce(format!(
                    "{} на тросе. Лети в гавань. Q / Z — длина троса.",
                    RELICS[index].name
                ));
            } else {
                self.announce("Подлети к лучу маяка и нажми E. Цель отмечена на компасе.");
            }
        }
    }

    pub fn release(&mut self, ship: &FlightBody) {
        if let Some(payload) = self.payload.take() {
            self.loose[payload.relic] = ship.position - Vec3::Y * 9.0;
            self.announce("Груз оставлен на аварийном поплавке. Его можно забрать снова.");
        }
    }

    pub fn damage(&mut self, ship: &mut FlightBody, amount: f32) {
        let actual = if ship.shielding {
            amount * 0.18
        } else {
            amount
        };
        ship.hull = (ship.hull - actual).max(0.0);
        self.damage_taken += actual;
        if ship.hull <= 0.0 {
            self.phase = Phase::Lost;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_three_deliveries_finish_the_expedition() {
        let mut game = Mission {
            phase: Phase::Flying,
            ..default()
        };
        let mut ship = FlightBody::default();
        for stage in 0..3 {
            ship.position = game.target();
            game.interact(&mut ship);
            assert_eq!(game.payload.as_ref().unwrap().relic, stage);
            ship.position = HOME;
            game.interact(&mut ship);
            assert_eq!(game.delivered, stage + 1);
        }
        assert_eq!(game.phase, Phase::Won);
    }

    #[test]
    fn pickup_requires_proximity_and_safe_speed() {
        let mut game = Mission {
            phase: Phase::Flying,
            ..default()
        };
        let mut ship = FlightBody::default();
        game.interact(&mut ship);
        assert!(game.payload.is_none());
        ship.position = game.target();
        ship.velocity = Vec3::X * 25.0;
        game.interact(&mut ship);
        assert!(game.payload.is_none());
    }

    #[test]
    fn released_cargo_can_be_recovered() {
        let mut game = Mission {
            phase: Phase::Flying,
            ..default()
        };
        let mut ship = FlightBody::default();
        ship.position = game.target();
        game.interact(&mut ship);
        game.release(&ship);
        ship.position = game.target();
        game.interact(&mut ship);
        assert!(game.payload.is_some());
    }

    #[test]
    fn pausing_prevents_interaction_and_shield_reduces_damage() {
        let mut game = Mission {
            phase: Phase::Paused,
            ..default()
        };
        let mut ship = FlightBody {
            shielding: true,
            ..default()
        };
        ship.position = game.target();
        game.interact(&mut ship);
        assert!(game.payload.is_none());
        game.damage(&mut ship, 20.0);
        assert!((ship.hull - 96.4).abs() < 0.01);
    }
}
