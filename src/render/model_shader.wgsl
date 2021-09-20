struct VertexInput {
  [[location(0)]] position: vec3<f32>;
  [[location(1)]] colour: vec3<f32>;
};

struct InstanceInput {
  [[location(2)]] position: vec3<f32>;
};

struct VertexOutput {
  [[builtin(position)]] clip_position: vec4<f32>;
};

[[stage(vertex)]]
fn vertex_main(vert: VertexInput, instance: InstanceInput) -> VertexOutput {
  var out: VertexOutput;
  let pos = (vert.position + instance.position) * 0.1;
  out.clip_position = vec4<f32>(pos, 1.0);
  return out;
}

[[stage(fragment)]]
fn fragment_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
