
struct VertexInput {
  @location(0) position: vec2<f32>,
}

struct FragmentInput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) sample_position: vec2<f32>,
}

struct EffectsUniform {
  multiply_colour: vec4<f32>,
  blur_radius: f32,
}

@group(0) @binding(0)
var prev_texture: texture_2d<f32>;
@group(0) @binding(1)
var prev_sampler: sampler;
@group(0) @binding(2)
var<uniform> effects: EffectsUniform;

@vertex
fn vertex_main(vert: VertexInput) -> FragmentInput {
  var out: FragmentInput;
  out.clip_position = vec4<f32>(
    vert.position.x * 2.0 - 1.0,
    vert.position.y * -2.0 + 1.0,
    1.0,
    1.0
  );
  out.sample_position = vec2<f32>(
    vert.position.x,
    vert.position.y,
  );
  return out;
}

@fragment
fn fragment_main(in: FragmentInput) -> @location(0) vec4<f32> {
  let prev: vec3<f32> = textureSample(prev_texture, prev_sampler, in.sample_position).rgb;
  // let prev_blurred = textureSampleLevel(prev_texture, prev_sampler, in.sample_position, 1.0).rgb;

  var final_colour: vec3<f32> = prev;

  if (effects.blur_radius > 0.0) {
    let blur_segments: i32 = 16;
    let blur_rings: i32 = 3;
    var blurred: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    for (var ring: i32 = 0; ring < blur_rings; ring++) {
      let ring_radius: f32 = effects.blur_radius * f32(ring + 1) / f32(blur_rings + 1);
      for (var segment: i32 = 0; segment < blur_segments; segment++) {
        let angle = f32(segment) / f32(blur_segments) * 6.28318530718 + f32(ring);
        let offset: vec2<f32> = ring_radius * vec2<f32>(sin(angle), cos(angle));
        var sample: vec3<f32> = textureSample(prev_texture, prev_sampler, in.sample_position + offset).rgb;
        blurred += sample;
      }
    }
    blurred /= f32(blur_segments * blur_rings);
    final_colour = blurred;
  }
  
  
  final_colour = mix(final_colour, prev * effects.multiply_colour.rgb, effects.multiply_colour.a);
  return clamp(vec4<f32>(final_colour, 1.0), vec4<f32>(0.0), vec4<f32>(1.0));
}
