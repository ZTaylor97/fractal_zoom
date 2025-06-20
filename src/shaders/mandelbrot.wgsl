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

struct Uniforms {
    time: f32,
    zoom: f32,
    offset: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(vertex_out: VertexOut) -> @location(0) vec4<f32> {
    let zoom = pow(0.8, -25* cos(uniforms.time * 0.05)+ 20);
    // let center = vec2(-0.97, -0.252);
    let center = vec2(-1.006, -0.2506);
    let uv = (vertex_out.uv - vec2(0.5)) * zoom;

    // Instead of adding center, subtract it to bring it to (0,0)
    let c = uv + center;


    var z = vec2<f32>(0.0);
    var i = 0u;

    let max_iter = u32(1000); 

    loop {
        if (i >= max_iter || dot(z, z) > 4.0) {
            break;
        }
        z = vec2<f32>(
            z.x * z.x - z.y * z.y + c.x,
            2.0 * z.x * z.y + c.y
        );
        i = i + 1u;
    }

    let t = f32(i) / f32(max_iter);

    return vec4<f32>(t*0.9 * sin(uniforms.time * 0.05), t * 0.3, t, 1.0);
}
