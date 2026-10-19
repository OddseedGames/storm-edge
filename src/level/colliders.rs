use super::{HOME, ISLANDS, hash};
use bevy::prelude::*;
use std::{
    f32::consts::{PI, TAU},
    sync::OnceLock,
};

pub enum Solid {
    Column {
        center: Vec3,
        radius: f32,
        half_height: f32,
    },
    Box {
        center: Vec3,
        half: Vec3,
    },
    Arc {
        center: Vec3,
        rotation: Quat,
        radius: f32,
        half_width: f32,
        half_depth: f32,
        start: f32,
        end: f32,
    },
}

impl Solid {
    pub fn penetration(&self, point: Vec3, margin: f32) -> Option<Vec3> {
        match *self {
            Self::Column {
                center,
                radius,
                half_height,
            } => {
                let p = point - center;
                let horizontal = Vec2::new(p.x, p.z).length();
                let side = radius + margin - horizontal;
                let vertical = half_height + margin - p.y.abs();
                if side <= 0.0 || vertical <= 0.0 {
                    return None;
                }
                if vertical < side {
                    Some(Vec3::Y * vertical * p.y.signum())
                } else {
                    Some(if horizontal > 0.001 {
                        Vec3::new(p.x, 0.0, p.z) / horizontal * side
                    } else {
                        Vec3::X * side
                    })
                }
            }
            Self::Box { center, half } => {
                let p = point - center;
                let overlap = half + Vec3::splat(margin) - p.abs();
                if overlap.min_element() <= 0.0 {
                    return None;
                }
                let axis = if overlap.x < overlap.y && overlap.x < overlap.z {
                    0
                } else if overlap.y < overlap.z {
                    1
                } else {
                    2
                };
                let mut push = Vec3::ZERO;
                push[axis] = overlap[axis] * if p[axis] >= 0.0 { 1.0 } else { -1.0 };
                Some(push)
            }
            Self::Arc {
                center,
                rotation,
                radius,
                half_width,
                half_depth,
                start,
                end,
            } => {
                let local = rotation.inverse() * (point - center);
                let horizontal = Vec2::new(local.x, local.y);
                let length = horizontal.length();
                let mut angle = local.y.atan2(local.x);
                if angle < start {
                    angle += TAU;
                }
                if angle > end {
                    return None;
                }
                let radial = half_width + margin - (length - radius).abs();
                let depth = half_depth + margin - local.z.abs();
                if radial <= 0.0 || depth <= 0.0 {
                    return None;
                }
                let push = if depth < radial {
                    Vec3::Z * depth * if local.z >= 0.0 { 1.0 } else { -1.0 }
                } else {
                    Vec3::new(local.x, local.y, 0.0).normalize_or_zero()
                        * radial
                        * if length >= radius { 1.0 } else { -1.0 }
                };
                Some(rotation * push)
            }
        }
    }
}

pub fn architecture() -> &'static [Solid] {
    static SOLIDS: OnceLock<Vec<Solid>> = OnceLock::new();
    SOLIDS.get_or_init(|| {
        let mut solids = Vec::new();
        for (index, island) in ISLANDS.iter().enumerate().take(4).skip(1) {
            let p = island.center + Vec3::Y * 6.0;
            let count = if index == 3 { 12 } else { 7 };
            for i in 0..count {
                let angle = i as f32 / count as f32 * TAU;
                let point = p + Vec3::new(
                    angle.cos() * island.radius * 0.28,
                    0.0,
                    angle.sin() * island.radius * 0.28,
                );
                let height = 9.0 + hash(i as f32 + island.seed) * 15.0;
                solids.push(Solid::Column {
                    center: point + Vec3::Y * height * 0.5,
                    radius: 1.7,
                    half_height: height * 0.5 + 0.7,
                });
            }
            if index < 3 {
                let center = p + Vec3::new(-island.radius * 0.55, 16.0, 0.0);
                solids.push(Solid::Arc {
                    center,
                    rotation: Quat::from_rotation_y(0.3),
                    radius: 16.0,
                    half_width: 1.2,
                    half_depth: 2.0,
                    start: -0.3,
                    end: PI + 0.3,
                });
                for side in [-1.0, 1.0] {
                    solids.push(Solid::Box {
                        center: center + Vec3::new(side * 16.0, -9.0, 0.0),
                        half: Vec3::new(1.5, 9.0, 2.0),
                    });
                }
            }
        }
        let p = HOME - Vec3::Y * 16.0;
        solids.push(Solid::Box {
            center: p,
            half: Vec3::new(14.0, 1.0, 21.0),
        });
        solids.push(Solid::Box {
            center: p + Vec3::new(0.0, 7.5, 26.0),
            half: Vec3::new(9.5, 7.5, 8.0),
        });
        solids.push(Solid::Column {
            center: p + Vec3::new(0.0, 26.5, 30.0),
            radius: 2.6,
            half_height: 12.5,
        });
        solids.push(Solid::Arc {
            center: HOME - Vec3::Z * 12.0,
            rotation: Quat::IDENTITY,
            radius: 14.0,
            half_width: 0.25,
            half_depth: 0.25,
            start: 0.0,
            end: TAU,
        });
        solids
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arch_has_a_flyable_opening_and_solid_rim() {
        let arc = Solid::Arc {
            center: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            radius: 16.0,
            half_width: 1.2,
            half_depth: 2.0,
            start: 0.0,
            end: PI,
        };
        assert!(arc.penetration(Vec3::Y * 8.0, 2.0).is_none());
        assert!(arc.penetration(Vec3::Y * 16.0, 2.0).is_some());
        assert!(arc.penetration(-Vec3::Y * 16.0, 2.0).is_none());
    }
    #[test]
    fn columns_resolve_side_and_cap_contacts() {
        let column = Solid::Column {
            center: Vec3::ZERO,
            radius: 2.0,
            half_height: 10.0,
        };
        assert!(column.penetration(Vec3::new(1.0, 0.0, 0.0), 1.0).unwrap().x > 0.0);
        assert!(
            column
                .penetration(Vec3::new(0.0, 10.5, 0.0), 1.0)
                .unwrap()
                .y
                > 0.0
        );
        assert!(column.penetration(Vec3::new(0.0, 12.0, 0.0), 1.0).is_none());
    }
}
