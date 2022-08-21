#import bevy_pbr::mesh_view_bindings

struct CustomMaterial {
    color: vec4<f32>,
    time: f32,
    offset: f32,
    selected: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

fn Muzzle(uv: vec2<f32>) -> vec4<f32> {
    var u =  vec2<f32>(1.0,.5) - uv;
    let T = floor(material.time * 20.);
    let theta = atan2(u.y,u.x);
    let len = (10. + sin(theta * 20. - T * 35.)) / 11.;
    u.y *=  4.;
    let d = max(-0.6, 1. - length(u)/len);
    return d*(1.+.5* vec4( sin(theta * 10. + T * 10.77),
                          -cos(theta *  8. - T *  8.77),
                          -sin(theta *  6. - T *134.77),
                           0.0));
}

fn cubicPulse(c: f32, w: f32, x: f32) -> f32{
   var x = x;
    x = abs(x - c);
    if( x>w ) {
        return 0.0;
    }
    x /= w;
    return 1.0 - x * x * (3.0 - 2.0 * x);
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let o = material.offset;
    let t = material.time * (4.0 + o / 10.0);

    var c = vec3(0.0, 0.0, 0.0);
    var a = 0.0;
    for(var i = 0.2; i<1.0; i+=0.2) {
        let m = sin(uv.x * (30.0 + o / 5.0) - t + o) * 0.1 + 0.5;
        let f = 1.0 / (50.0 * abs(m - uv.y));

        c += f * material.color.xyz;
        a += f;
    }

    a = smoothstep(0.2, 0.4, a) * 0.7;

    // add a white border to the selected one
    var sel = material.selected * cubicPulse(0.3, 0.1, a);
    a += sel;
    c += vec3(sel);

    return vec4(c, a);
}
