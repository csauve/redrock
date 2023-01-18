
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
    var blurred: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    let n: i32 = 48;
    for (var i: i32 = 0; i < 48; i++) {
      let theta: f32 = 2.39996322973 * f32(i); //2pi/phi^2
      let r: f32 = sqrt(f32(i)) * effects.blur_radius;
      let offset: vec2<f32> = r * vec2<f32>(sin(theta), cos(theta));
      var sample: vec3<f32> = textureSample(prev_texture, prev_sampler, in.sample_position + offset).rgb;
      blurred += sample;
    }
    blurred /= f32(n);
    final_colour = blurred;
  }
  
  final_colour = mix(final_colour, prev * effects.multiply_colour.rgb, effects.multiply_colour.a);
  // final_colour = smoothstep(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(2.0, 2.0, 2.0), final_colour);
  return vec4<f32>(final_colour, 1.0); //clamped by Bgra8UnormSrgb
}
