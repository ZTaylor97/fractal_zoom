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
};
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

fn get_colour( intensity: f32) -> vec3<f32> {
   let t0: f32 = 0.0;
   let t1: f32 = 0.25;
   let t2: f32 = 0.50;
   let t3: f32 = 0.85;
   let t4: f32 = 1.0;

    let c0 = vec3(0.0, 0.0, 0.0);
    let c1 = vec3(0.2, 0.0, 0.6);
    let c2 = vec3(0.8, 0.2, 0.1);
    let c3 = vec3(1.0, 0.8, 0.1);
    let c4 = vec3(1.0, 1.0, 0.5);

    if (intensity < t1){
        return mix(c0, c1, (intensity - t0) / (t1 - t0));
    }
    else if (intensity < t2) {
        return mix(c1, c2, (intensity - t1) / (t2 - t1));
    }
    else if (intensity < t3){
        return mix(c2, c3, (intensity - t2) / (t3 - t2));
    }
    else {
        return mix(c3, c4, (intensity - t3) / (t4 - t3));
    }
}


@fragment
fn fs_main(vertex_out: VertexOut) -> @location(0) vec4<f32> {
    // Map UV from [0, 1] to [-1.5, 1.5] and [-1.0, 1.0]
    // let zoom = pow(1.2, uniforms.zoom);
    // let bound_x = f32(1.5 / zoom);
    // let bound_y = f32(1.5 / zoom);

    // let offset = vec2<f32>(uniforms.offset.x, uniforms.offset.y);

    // let z = vec2<f32>(
    //     mix(-bound_x, bound_x, vertex_out.uv.x + offset.x),
    //     mix(-bound_y, bound_y, vertex_out.uv.y - offset.y)
    // );

    let zoom = pow(1.2, uniforms.zoom);
    let bound_x = 1.5 / zoom;
    let bound_y = 1.5 / zoom;

    let uv = vertex_out.uv * 2.0 - vec2(1.0, 1.0);
    let z = uv * vec2(bound_x, bound_y) - vec2(uniforms.offset.x, -uniforms.offset.y);

    let c = vec2<f32>(
        0.7885 * cos(uniforms.time * 0.05),
        0.7885 * sin(uniforms.time * 0.05)
    );

    var value = z;
    var i = 0u;
    let iterations = u32(100 + 100 * zoom);
    loop {
        if (i >= iterations || dot(value, value) > 4.0) {
            break;
        }
        value = vec2<f32>(
            value.x * value.x - value.y * value.y + c.x,
            2 * value.x * value.y + c.y
        );
        i = i + 1u;
    }


    var t = f32(i) - log2(log2(dot(value,value))) + 4;
    t = clamp(t / f32(iterations), 0.0, 1.0);

    let colour = get_colour(t);
    return vec4<f32>(colour, 1.0);
}
