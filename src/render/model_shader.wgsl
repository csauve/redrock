struct VertexInput {
  [[location(0)]] position: vec3<f32>;
  [[location(1)]] colour: vec3<f32>;
};

struct InstanceInput {
  [[location(2)]] model_matrix_0: vec4<f32>;
  [[location(3)]] model_matrix_1: vec4<f32>;
  [[location(4)]] model_matrix_2: vec4<f32>;
  [[location(5)]] model_matrix_3: vec4<f32>;
};

struct VertexOutput {
  [[builtin(position)]] clip_position: vec4<f32>;
};

[[block]]
struct CameraUniform {
  view_proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraUniform;

[[stage(vertex)]]
fn vertex_main(vert: VertexInput, instance: InstanceInput) -> VertexOutput {
  var out: VertexOutput;

  let model_matrix = mat4x4<f32>(
    instance.model_matrix_0,
    instance.model_matrix_1,
    instance.model_matrix_2,
    instance.model_matrix_3
  );

  out.clip_position = camera.view_proj * model_matrix * vec4<f32>(vert.position, 1.0);
  
  return out;
}

[[stage(fragment)]]
fn fragment_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
