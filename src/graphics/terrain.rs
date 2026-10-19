use super::geometry::Geometry;
use crate::level::{Island, hash};
use bevy::prelude::*;

pub fn island(island: Island) -> Geometry {
    let mut mesh = Geometry::default();
    let sides = 72;
    let rings = 14;
    let top = |ring: usize, side: usize| {
        let angle = (side % sides) as f32 / sides as f32 * std::f32::consts::TAU;
        let radius = island.rim(angle) * ring as f32 / rings as f32;
        let x = island.center.x + angle.cos() * radius;
        let z = island.center.z + angle.sin() * radius;
        Vec3::new(x, island.surface(x, z), z)
    };
    for ring in 0..rings {
        for side in 0..sides {
            let noise = hash(ring as f32 * 71.0 + side as f32 + island.seed);
            let color = if ring > rings - 3 {
                [0.34 + noise * 0.13, 0.36 + noise * 0.1, 0.29 + noise * 0.06]
            } else {
                [
                    0.19 + noise * 0.08,
                    0.34 + noise * 0.11,
                    0.21 + noise * 0.06,
                ]
            };
            mesh.quad(
                top(ring, side),
                top(ring, side + 1),
                top(ring + 1, side + 1),
                top(ring + 1, side),
                color,
            );
        }
    }
    let cliff = |ring: usize, side: usize| {
        let angle = (side % sides) as f32 / sides as f32 * std::f32::consts::TAU;
        let fraction = ring as f32 / 10.0;
        let ridge = (angle * 13.0 + island.seed).sin() * 0.035 + (angle * 23.0).cos() * 0.018;
        let jitter = 1.0 + (fraction * 17.0 + angle * 7.0 + island.seed).sin() * 0.035;
        let radius = island.rim(angle)
            * ((1.0 - fraction).powf(0.58) + ridge * fraction).max(0.025)
            * if ring == 0 { 1.0 } else { jitter };
        let x = island.center.x + angle.cos() * radius;
        let z = island.center.z + angle.sin() * radius;
        Vec3::new(
            x,
            if ring == 0 {
                island.surface(x, z)
            } else {
                island.center.y - fraction * island.depth
            },
            z,
        )
    };
    for ring in 0..10 {
        for side in 0..sides {
            let t = hash(side as f32 * 13.0 + ring as f32 + island.seed);
            let band = (ring as f32 * 1.8 + island.seed).sin() * 0.025;
            let color = [
                0.32 + t * 0.07 + band,
                0.39 + t * 0.06 + band,
                0.36 + t * 0.06 + band,
            ];
            mesh.quad(
                cliff(ring, side),
                cliff(ring, side + 1),
                cliff(ring + 1, side + 1),
                cliff(ring + 1, side),
                color,
            );
        }
    }
    let bottom = island.center - Vec3::Y * island.depth;
    for side in 0..sides {
        mesh.triangle(
            bottom,
            cliff(10, side),
            cliff(10, side + 1),
            [0.3, 0.36, 0.33],
        );
    }
    for i in 0..20 {
        let angle = hash(i as f32 + island.seed) * std::f32::consts::TAU;
        let radius = island.radius * (0.91 + hash(i as f32 + 18.0) * 0.06);
        let height = 6.0 + hash(i as f32 * 3.0 + island.seed) * 19.0;
        let point =
            island.center + Vec3::new(angle.cos() * radius, -height * 0.3, angle.sin() * radius);
        mesh.cylinder(point, 1.0 + height * 0.08, height, 6, [0.31, 0.34, 0.32]);
    }
    mesh.smooth()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cliff_faces_point_outward_and_bottom_is_closed() {
        for island in crate::level::ISLANDS {
            let mesh = island_geometry_for_test(island);
            let top_vertices = 14 * 72 * 6;
            let cliff_vertices = 10 * 72 * 6;
            for triangle in
                mesh.positions[top_vertices..top_vertices + cliff_vertices].chunks_exact(3)
            {
                let [a, b, c] = [
                    Vec3::from_array(triangle[0]),
                    Vec3::from_array(triangle[1]),
                    Vec3::from_array(triangle[2]),
                ];
                let normal = (b - a).cross(c - a);
                let center = (a + b + c) / 3.0 - island.center;
                let radial = Vec3::new(center.x, 0.0, center.z);
                assert!(normal.dot(radial) > 0.0, "inward face at {center:?}");
            }
            let bottom_start = top_vertices + cliff_vertices;
            for triangle in mesh.positions[bottom_start..bottom_start + 72 * 3].chunks_exact(3) {
                let [a, b, c] = [
                    Vec3::from_array(triangle[0]),
                    Vec3::from_array(triangle[1]),
                    Vec3::from_array(triangle[2]),
                ];
                assert!((b - a).cross(c - a).y < 0.0);
            }
        }
    }

    fn island_geometry_for_test(definition: Island) -> Geometry {
        island(definition)
    }

    #[test]
    fn island_shell_has_no_open_edges() {
        for definition in crate::level::ISLANDS {
            let mesh = island(definition);
            let shell_vertices = 14 * 72 * 6 + 10 * 72 * 6 + 72 * 3;
            let key = |p: [f32; 3]| {
                (
                    (p[0] * 100.0).round() as i32,
                    (p[1] * 100.0).round() as i32,
                    (p[2] * 100.0).round() as i32,
                )
            };
            let mut edges = std::collections::HashMap::new();
            for triangle in mesh.positions[..shell_vertices].chunks_exact(3) {
                let a = Vec3::from_array(triangle[0]);
                let b = Vec3::from_array(triangle[1]);
                let c = Vec3::from_array(triangle[2]);
                if (b - a).cross(c - a).length_squared() < 0.00001 {
                    continue;
                }
                for (a, b) in [
                    (triangle[0], triangle[1]),
                    (triangle[1], triangle[2]),
                    (triangle[2], triangle[0]),
                ] {
                    let a = key(a);
                    let b = key(b);
                    let edge = if a < b { (a, b) } else { (b, a) };
                    *edges.entry(edge).or_insert(0) += 1;
                }
            }
            assert!(
                edges.values().all(|count| *count == 2),
                "open shell for seed {}",
                definition.seed
            );
        }
    }
}

pub fn vegetation(island: Island) -> Geometry {
    let mut mesh = Geometry::default();
    for i in 0..62 {
        let k = i as f32 + island.seed * 101.0;
        let angle = hash(k) * std::f32::consts::TAU;
        let radius = island.radius * (0.25 + hash(k + 2.0) * 0.65);
        let x = island.center.x + angle.cos() * radius;
        let z = island.center.z + angle.sin() * radius;
        let p = Vec3::new(x, island.surface(x, z), z);
        let h = 3.0 + hash(k + 4.0) * 8.0;
        let lean = Vec3::new(hash(k + 6.0) - 0.5, 0.0, hash(k + 7.0) - 0.5) * 1.8;
        mesh.beam(
            p,
            p + Vec3::Y * h + lean,
            0.32 + h * 0.03,
            [0.23, 0.22, 0.16],
        );
        let leaf = if i % 7 == 0 {
            [0.67, 0.42, 0.17]
        } else {
            [0.19, 0.4 + hash(k) * 0.11, 0.25]
        };
        for branch in 0..3 {
            let t = branch as f32 * 2.3 + k;
            let crown = p
                + Vec3::Y * (h - branch as f32 * 0.8)
                + lean
                + Vec3::new(t.sin(), 0.0, t.cos()) * h * 0.17;
            mesh.ellipsoid(
                crown,
                Vec3::new(h * 0.31, h * 0.2, h * 0.3),
                leaf,
                k + branch as f32,
            );
        }
    }
    for i in 0..42 {
        let k = i as f32 * 17.0 + island.seed;
        let angle = hash(k) * std::f32::consts::TAU;
        let radius = island.radius * hash(k + 3.0) * 0.94;
        let x = island.center.x + angle.cos() * radius;
        let z = island.center.z + angle.sin() * radius;
        let p = Vec3::new(x, island.surface(x, z), z);
        let size = 0.6 + hash(k + 4.0) * 2.4;
        mesh.ellipsoid(
            p,
            Vec3::new(size, size * 0.65, size * 0.8),
            [0.38, 0.41, 0.34],
            k,
        );
    }
    mesh.smooth()
}
