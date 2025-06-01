// mandelbrot.wgsl

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

@fragment
fn fs_main(vertex_out: VertexOut) -> @location(0) vec4<f32> {
    let c = vec2<f32>(
        mix(-2.0, 1.0, vertex_out.uv.x),
        mix(-1.5, 1.5, vertex_out.uv.y),
    );

    var z = vec2<f32>(0.0);
    var i = 0u;
    loop {
        if (i >= 100u || dot(z, z) > 4.0) {
            break;
        }
        z = vec2<f32>(
            z.x * z.x - z.y * z.y + c.x,
            2.0 * z.x * z.y + c.y
        );
        i = i + 1u;
    }

    let t = f32(i) / 100.0;
    return vec4<f32>(t, t * 0.5, t * 0.25, 1.0);
}
