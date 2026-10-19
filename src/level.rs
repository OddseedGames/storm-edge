use bevy::prelude::*;
pub mod colliders;

pub const HOME: Vec3 = Vec3::new(0.0, 38.0, 138.0);
pub const START: Vec3 = Vec3::new(0.0, 42.0, 98.0);
pub const WORLD_RADIUS: f32 = 650.0;

#[derive(Clone, Copy)]
pub struct Island {
    pub center: Vec3,
    pub radius: f32,
    pub depth: f32,
    pub seed: f32,
}

impl Island {
    pub fn rim(&self, angle: f32) -> f32 {
        self.radius
            * (1.0
                + (angle * 5.0 + self.seed).sin() * 0.12
                + (angle * 9.0 - self.seed).cos() * 0.065)
    }

    pub fn surface(&self, x: f32, z: f32) -> f32 {
        let p = Vec2::new(x - self.center.x, z - self.center.z);
        let radial = (p.length() / self.radius).min(1.2);
        self.center.y
            + 5.0 * (1.0 - radial * radial).max(0.0)
            + (p.x * 0.11 + self.seed).sin() * (p.y * 0.1).cos() * 1.7
    }

    pub fn penetration(&self, position: Vec3, margin: f32) -> Option<Vec3> {
        let delta = position - self.center;
        let horizontal = Vec2::new(delta.x, delta.z).length();
        let angle = delta.z.atan2(delta.x);
        let top = self.surface(position.x, position.z);
        let taper = ((position.y - self.center.y + self.depth) / self.depth)
            .clamp(0.0, 1.0)
            .sqrt();
        let radius = self.rim(angle) * taper + margin;
        if position.y < top + margin
            && position.y > self.center.y - self.depth - margin
            && horizontal < radius
        {
            let up = top + margin - position.y;
            let side = radius - horizontal;
            if up < side || horizontal < 0.01 {
                Some(Vec3::Y * up)
            } else {
                Some(Vec3::new(delta.x, 0.0, delta.z).normalize() * side)
            }
        } else {
            None
        }
    }
}

pub const ISLANDS: [Island; 8] = [
    Island {
        center: Vec3::new(0.0, 15.0, 156.0),
        radius: 47.0,
        depth: 88.0,
        seed: 1.3,
    },
    Island {
        center: Vec3::new(-98.0, 24.0, -4.0),
        radius: 49.0,
        depth: 112.0,
        seed: 4.8,
    },
    Island {
        center: Vec3::new(97.0, 49.0, -140.0),
        radius: 59.0,
        depth: 130.0,
        seed: 8.3,
    },
    Island {
        center: Vec3::new(-22.0, 78.0, -339.0),
        radius: 74.0,
        depth: 165.0,
        seed: 12.7,
    },
    Island {
        center: Vec3::new(134.0, -1.0, 68.0),
        radius: 23.0,
        depth: 73.0,
        seed: 19.0,
    },
    Island {
        center: Vec3::new(-195.0, 65.0, -171.0),
        radius: 32.0,
        depth: 115.0,
        seed: 25.0,
    },
    Island {
        center: Vec3::new(220.0, 99.0, -305.0),
        radius: 42.0,
        depth: 140.0,
        seed: 33.0,
    },
    Island {
        center: Vec3::new(-270.0, 12.0, 60.0),
        radius: 48.0,
        depth: 130.0,
        seed: 41.0,
    },
];

#[derive(Clone, Copy)]
pub struct Relic {
    pub name: &'static str,
    pub position: Vec3,
    pub mass: f32,
    pub color: [f32; 3],
}

pub const RELICS: [Relic; 3] = [
    Relic {
        name: "Сердце сада",
        position: Vec3::new(-87.0, 43.0, 1.0),
        mass: 70.0,
        color: [0.3, 1.0, 0.8],
    },
    Relic {
        name: "Осколок солнца",
        position: Vec3::new(90.0, 69.0, -129.0),
        mass: 105.0,
        color: [1.0, 0.65, 0.22],
    },
    Relic {
        name: "Ядро бури",
        position: Vec3::new(-22.0, 103.0, -331.0),
        mass: 145.0,
        color: [0.6, 0.5, 1.0],
    },
];

pub fn hash(n: f32) -> f32 {
    ((n * 127.1).sin() * 43_758.547).fract().abs()
}
