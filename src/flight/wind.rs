use bevy::prelude::*;

pub fn sample(position: Vec3, seconds: f32, storm: f32) -> Vec3 {
    let gust = (seconds * 0.42 + position.z * 0.015).sin();
    let cross = (seconds * 0.27 + position.x * 0.021).cos();
    let thermal_center = Vec2::new(32.0, -72.0);
    let thermal =
        (-Vec2::new(position.x, position.z).distance_squared(thermal_center) / 2300.0).exp();
    Vec3::new(
        2.0 + gust * (1.2 + storm * 3.0),
        thermal * 5.0,
        cross * (1.8 + storm * 4.0),
    )
}
