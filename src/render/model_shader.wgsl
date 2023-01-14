struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) tangent: vec3<f32>,
  @location(3) bitangent: vec3<f32>,
  @location(4) uv: vec2<f32>,
}

struct InstanceInput {
  @location(5) model_matrix_0: vec4<f32>,
  @location(6) model_matrix_1: vec4<f32>,
  @location(7) model_matrix_2: vec4<f32>,
  @location(8) model_matrix_3: vec4<f32>,
  @location(9) normal_matrix_0: vec3<f32>,
  @location(10) normal_matrix_1: vec3<f32>,
  @location(11) normal_matrix_2: vec3<f32>,
  @location(12) colour: vec3<f32>,
}

struct FragmentInput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) world_position: vec3<f32>,
  @location(1) tangent_light: vec3<f32>,
  @location(2) tangent_eye: vec3<f32>,
  @location(3) instance_colour: vec3<f32>,
  @location(4) uv: vec2<f32>,
}

struct CameraUniform {
  view_proj: mat4x4<f32>,
  world_position: vec3<f32>,
}

struct EnvironmentUniform {
  fog_colour: vec4<f32>,
  fog_min_distance: f32,
  fog_max_distance: f32,
  sun_colour: vec3<f32>,
  sun_direction: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(0) @binding(1)
var<uniform> environment: EnvironmentUniform;

@vertex
fn vertex_main(vert: VertexInput, instance: InstanceInput) -> FragmentInput {
  let model_matrix = mat4x4<f32>(
    instance.model_matrix_0,
    instance.model_matrix_1,
    instance.model_matrix_2,
    instance.model_matrix_3
  );

  let normal_matrix = mat3x3<f32>(
    instance.normal_matrix_0,
    instance.normal_matrix_1,
    instance.normal_matrix_2,
  );

  let world_position: vec4<f32> = model_matrix * vec4<f32>(vert.position, 1.0);
  let world_normal: vec3<f32> = normal_matrix * vert.normal;
  let world_tangent: vec3<f32> = normal_matrix * vert.tangent;
  let world_bitangent: vec3<f32> = normal_matrix * vert.bitangent;
  let tangent_matrix: mat3x3<f32> = transpose(mat3x3<f32>(
    world_tangent,
    world_bitangent,
    world_normal,
  ));

  let world_light: vec3<f32> = environment.sun_direction;
  let world_eye: vec3<f32> = normalize(camera.world_position - world_position.xyz);

  var out: FragmentInput;
  out.clip_position = camera.view_proj * world_position;
  out.world_position = world_position.xyz;
  out.tangent_light = tangent_matrix * world_light;
  out.tangent_eye = tangent_matrix * world_eye;
  out.uv = vert.uv;
  out.instance_colour = instance.colour;
  return out;
}

@fragment
fn fragment_main(in: FragmentInput) -> @location(0) vec4<f32> {
  let tangent_normal = vec3<f32>(0.0, 0.0, 1.0);
  let nl: f32 = dot(tangent_normal, in.tangent_light);

  //fog
  let dist = abs(length(camera.world_position - in.world_position));
  let fog_amt: f32 = clamp((dist - environment.fog_min_distance) / environment.fog_max_distance, 0.0, environment.fog_colour.a);
  let fog_colour: vec3<f32> = environment.fog_colour.rgb;

  //diffuse
  let diffuse_colour: vec3<f32> = in.instance_colour;
  let ambient_amt: f32 = 0.1;
  let ambient: vec3<f32> = diffuse_colour * fog_colour * ambient_amt;
  let diffuse: vec3<f32> = diffuse_colour * environment.sun_colour * saturate(nl) + ambient;

  //spec
  let specular_colour: vec3<f32> = environment.sun_colour;
  let specular_amt = saturate(dot(-reflect(in.tangent_light, tangent_normal), in.tangent_eye));
  let specular: vec3<f32> = saturate(nl) * specular_colour * specular_amt * specular_amt;
  
  let final_colour: vec3<f32> = mix(diffuse + specular, fog_colour, fog_amt);
  return clamp(vec4<f32>(final_colour, 1.0), vec4<f32>(0.0), vec4<f32>(1.0));
}
