#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> parameters: vec4<f32>;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p,vec2<f32>(127.1,311.7)))*43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f*f*(3.0-2.0*f);
    return mix(mix(hash(i),hash(i+vec2<f32>(1.0,0.0)),u.x),
               mix(hash(i+vec2<f32>(0.0,1.0)),hash(i+vec2<f32>(1.0,1.0)),u.x),u.y);
}

fn fbm(p: vec2<f32>) -> f32 {
    var q=p;
    var result=0.0;
    var amplitude=0.5;
    for(var i=0;i<5;i=i+1){
        result+=noise(q)*amplitude;
        q=mat2x2<f32>(1.6,1.2,-1.2,1.6)*q+vec2<f32>(7.3,13.7);
        amplitude*=0.5;
    }
    return result;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t=parameters.x;
    let storm=parameters.y;
    if(parameters.w>1.5){
        let uv=in.uv;
        let streak=noise(vec2<f32>(uv.x*35.0,uv.y*8.0+t*2.2));
        let edge=smoothstep(0.0,0.16,uv.x)*(1.0-smoothstep(0.82,1.0,uv.x));
        let foam=pow(streak,3.0);
        return vec4<f32>(mix(vec3<f32>(0.35,0.73,0.76),vec3<f32>(1.8,2.1,1.9),foam),edge*0.64);
    }
    let ray=normalize(in.world_position.xyz-view.world_position.xyz);
    let height=clamp(ray.y,0.0,1.0);
    var color=mix(vec3<f32>(0.33,0.61,0.69),vec3<f32>(0.045,0.19,0.36),pow(height,0.6));
    let sunDirection=normalize(vec3<f32>(-0.52,0.62,-0.7));
    let sun=max(dot(ray,sunDirection),0.0);
    color+=vec3<f32>(1.0,0.65,0.32)*pow(sun,10.0)*0.3;
    color+=vec3<f32>(5.5,4.5,2.8)*pow(sun,1600.0);
    let uv=ray.xz/(max(ray.y,0.06)+0.28)*2.4+vec2<f32>(t*0.006,0.0);
    let cloud=smoothstep(0.53,0.8,fbm(uv))*smoothstep(-0.04,0.15,ray.y);
    color=mix(color,vec3<f32>(0.93,0.89,0.76),cloud*0.78);
    if(ray.y < -0.005){
        let travel=(view.world_position.y+62.0)/max(-ray.y,0.005);
        let hit=view.world_position.xz+ray.xz*travel;
        let p=hit*0.007+vec2<f32>(t*0.009,0.0);
        let n=fbm(p+vec2<f32>(fbm(p*0.65)));
        let detail=fbm(p*5.0);
        let rim=pow(smoothstep(0.32,0.75,n),1.5);
        var sea=mix(vec3<f32>(0.28,0.5,0.53),vec3<f32>(1.05,1.0,0.83),rim)+detail*0.065;
        let haze=1.0-exp(-travel*0.0007);
        sea=mix(sea,vec3<f32>(0.68,0.77,0.74),haze)*mix(1.0,0.63,storm);
        color=mix(color,sea,smoothstep(0.005,0.075,-ray.y));
    }
    let stormSide=smoothstep(0.35,0.9,-ray.z)*storm;
    color=mix(color,vec3<f32>(0.15,0.2,0.31),stormSide*0.65);
    return vec4<f32>(color,1.0);
}
