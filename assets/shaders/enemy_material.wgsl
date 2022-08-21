#import bevy_pbr::mesh_view_bindings

struct CustomMaterial {
    color: vec4<f32>,
    time: f32,
    damaged: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@group(1) @binding(1)
var symbol_texture: texture_2d<f32>;
@group(1) @binding(2)
var symbol_sampler: sampler;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    var c = material.color;
    c.a -= material.damaged * (sin(material.time * 6.0) * 0.2 + 0.2);

    let s = textureSample(symbol_texture, symbol_sampler, uv).a;
    let sc = vec4(vec3(mix(0.0, 1.0, material.damaged)), 1.0);

    c = mix(c, sc, s);

    return c;
}
