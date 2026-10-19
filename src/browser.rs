use crate::game::{Pilot, combat::Threats, mission::Mission};
use bevy::prelude::*;

pub fn publish(
    time: Res<Time>,
    pilot: Res<Pilot>,
    mission: Res<Mission>,
    threats: Res<Threats>,
    mut elapsed: Local<f32>,
) {
    *elapsed += time.delta_secs();
    if *elapsed < 0.1 {
        return;
    }
    *elapsed = 0.0;
    #[cfg(target_arch = "wasm32")]
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(status) = document.get_element_by_id("game-status") {
            let b = &pilot.body;
            let values = [
                ("phase", format!("{:?}", mission.phase)),
                ("x", format!("{:.3}", b.position.x)),
                ("y", format!("{:.3}", b.position.y)),
                ("z", format!("{:.3}", b.position.z)),
                ("yaw", format!("{:.4}", b.yaw)),
                ("yaw-rate", format!("{:.4}", b.yaw_rate)),
                ("vx", format!("{:.3}", b.velocity.x)),
                ("vy", format!("{:.3}", b.velocity.y)),
                ("vz", format!("{:.3}", b.velocity.z)),
                ("speed", format!("{:.2}", b.velocity.length())),
                ("hull", format!("{:.2}", b.hull)),
                ("energy", format!("{:.2}", b.energy)),
                ("heat", format!("{:.2}", b.heat)),
                ("delivered", mission.delivered.to_string()),
                (
                    "cargo",
                    mission
                        .payload
                        .as_ref()
                        .map_or(-1, |p| p.relic as i32)
                        .to_string(),
                ),
                (
                    "tether",
                    format!("{:.2}", mission.payload.as_ref().map_or(0.0, |p| p.length)),
                ),
                ("elapsed", format!("{:.2}", mission.elapsed)),
                ("fps", format!("{:.0}", 1.0 / time.delta_secs().max(0.0001))),
                ("notice", mission.notice.clone()),
                (
                    "threat",
                    threats
                        .hunters
                        .iter()
                        .filter(|h| h.health > 0.0 && h.alert > 0.2)
                        .count()
                        .to_string(),
                ),
                ("downed", threats.downed.to_string()),
                ("shots", mission.shots.to_string()),
                (
                    "incoming",
                    format!(
                        "{:.1}",
                        threats
                            .pulses
                            .iter()
                            .filter(|p| p.hostile)
                            .map(|p| p.position.distance(b.position))
                            .fold(999.0, f32::min)
                    ),
                ),
                ("shield", b.shielding.to_string()),
                ("boost", b.boosting.to_string()),
            ];
            for (key, value) in values {
                let _ = status.set_attribute(&format!("data-{key}"), &value);
            }
            let _ = status.set_attribute("data-ready", "true");
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (&pilot, &mission, &threats);
}
