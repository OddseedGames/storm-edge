#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var rock_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var rock_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(102) var<uniform> settings: vec4<f32>;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2<f32>(1.0, 0.0)), u.x),
               mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x), u.y);
}

fn sample_rock(uv: vec2<f32>) -> vec3<f32> {
    let grid = vec2<f32>(uv.x - uv.y * 0.57735, uv.y * 1.1547);
    let cell = floor(grid);
    let f = fract(grid);
    var a = cell;
    let b = cell + vec2<f32>(1.0, 0.0);
    let c = cell + vec2<f32>(0.0, 1.0);
    var w = vec3<f32>(1.0 - f.x - f.y, f.x, f.y);
    if f.x + f.y > 1.0 {
        a = cell + vec2<f32>(1.0, 1.0);
        w = vec3<f32>(f.x + f.y - 1.0, 1.0 - f.y, 1.0 - f.x);
    }
    w = w * w;
    w /= w.x + w.y + w.z;
    let da = vec2<f32>(hash(a), hash(a + vec2<f32>(7.0, 13.0))) * 17.0;
    let db = vec2<f32>(hash(b), hash(b + vec2<f32>(7.0, 13.0))) * 17.0;
    let dc = vec2<f32>(hash(c), hash(c + vec2<f32>(7.0, 13.0))) * 17.0;
    let dx = dpdx(uv);
    let dy = dpdy(uv);
    return textureSampleGrad(rock_texture, rock_sampler, uv + da, dx, dy).rgb * w.x
        + textureSampleGrad(rock_texture, rock_sampler, uv + db, dx, dy).rgb * w.y
        + textureSampleGrad(rock_texture, rock_sampler, uv + dc, dx, dy).rgb * w.z;
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) front: bool) -> FragmentOutput {
    var pbr = pbr_input_from_standard_material(in, front);
    let p = in.world_position.xyz;
    let normal = normalize(in.world_normal);
    let weight = pow(abs(normal), vec3<f32>(4.0));
    let blend = weight / (weight.x + weight.y + weight.z);
    let x = sample_rock(p.zy * settings.x + vec2<f32>(0.31, 0.71));
    let y = sample_rock(p.xz * settings.x + vec2<f32>(0.57, 0.11));
    let z = sample_rock(p.xy * settings.x + vec2<f32>(0.13, 0.43));
    let stone = (x * blend.x + y * blend.y + z * blend.z) * vec3<f32>(0.87, 0.94, 0.94);
    let variation = noise(p.xz * 0.026 + vec2<f32>(p.y * 0.013));
    let fine = noise(p.xz * 1.8);
    let grass = mix(vec3<f32>(0.025, 0.075, 0.03), vec3<f32>(0.085, 0.18, 0.055), variation) * (0.85 + fine * 0.3);
    let coverage = smoothstep(0.44, 0.83, normal.y + (variation - 0.5) * 0.22) * settings.y;
    pbr.material.base_color = vec4<f32>(mix(stone * (0.85 + variation * 0.25), grass, coverage), 1.0);
    pbr.material.perceptual_roughness = settings.z;
    let height = mix(dot(stone, vec3<f32>(0.2126, 0.7152, 0.0722)), fine * 0.06, coverage);
    let sx = dpdx(p);
    let sy = dpdy(p);
    let r1 = cross(sy, normal);
    let r2 = cross(normal, sx);
    let determinant = dot(sx, r1);
    let gradient = (dpdx(height) * r1 + dpdy(height) * r2) / max(abs(determinant), 0.000001) * sign(determinant);
    pbr.N = normalize(normal - clamp(gradient, vec3<f32>(-0.8), vec3<f32>(0.8)) * 0.16);
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr);
    out.color = main_pass_post_lighting_processing(pbr, out.color);
    return out;
}
