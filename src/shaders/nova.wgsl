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
    let zoom = pow(1.2, uniforms.zoom);
    let bound_x = 1.5 / zoom;
    let bound_y = 1.5 / zoom;

    let uv = vertex_out.uv * 2.0 - vec2(1.0, 1.0);
    var z = uv * vec2(bound_x, bound_y) - vec2(uniforms.offset.x, -uniforms.offset.y);

    // Small perturbation constant to animate over time
    let c = vec2<f32>(
        0.3 * cos(uniforms.time * 0.2),
        0.3 * sin(uniforms.time * 0.35)
    );

    var i = 0u;
    let max_iter = clamp(u32(100 + 100 * zoom), 0, 1000);
    loop {
        if (i >= max_iter) {
            break;
        }

        // f(z) = z^3 - 1
        let r2 = z.x * z.x - z.y * z.y;
        let i2 = 2.0 * z.x * z.y;
        let fz = vec2<f32>(
            r2 * z.x - 3.0 * z.x * z.y * z.y - 1.0,
            3.0 * z.x * z.x * z.y - z.y * z.y * z.y
        );

        // f'(z) = 3z^2
        let dfz = vec2<f32>(
            3.0 * (z.x * z.x - z.y * z.y),
            6.0 * z.x * z.y
        );

        // z = z - f(z)/f'(z) + c
        let denom = dot(dfz, dfz) + 1e-6;
        let correction = vec2<f32>(
            (fz.x * dfz.x + fz.y * dfz.y) / denom,
            (fz.y * dfz.x - fz.x * dfz.y) / denom
        );

        z = z - correction + c;

        if (dot(fz, fz) < 1e-6) {
            break;
        }

        i += 1u;
    }

    var t = f32(i) - log2(log2(dot(z,z) + 1e-6)) + 4.0;
    t = clamp(t / f32(max_iter), 0.0, 1.0);


    let colour = get_colour(t);
    return vec4<f32>(colour, 1.0);
}
