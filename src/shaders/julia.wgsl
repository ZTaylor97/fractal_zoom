struct VertexIn {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.uv = in.uv;
    return out;
}
struct Uniforms {
    time: f32,
};
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;


@fragment
fn fs_main(vertex_out: VertexOut) -> @location(0) vec4<f32> {
    // Map UV from [0, 1] to [-1.5, 1.5] and [-1.0, 1.0]
    let z = vec2<f32>(
        mix(-1.5, 1.5, vertex_out.uv.x),
        mix(-1.0, 1.0, vertex_out.uv.y)
    );

    // Animate 'c' using uniforms.time (c = constant for Julia)
    let c = vec2<f32>(
        0.7885 * cos(uniforms.time * 0.05),
        0.7885 * sin(uniforms.time * 0.05)
    );

    var value = z;
    var i = 0u;
    loop {
        if (i >= 255u || dot(value, value) > 4.0) {
            break;
        }
        value = vec2<f32>(
            value.x * value.x - value.y * value.y + c.x,
            2.0 * value.x * value.y + c.y
        );
        i = i + 1u;
    }

    let t = f32(i) / 255.0;
    return vec4<f32>(t * 0.9, t * 0.3, t, 1.0);
}
