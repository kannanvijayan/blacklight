use crate::{
  api::{
    Project,
    data_type::{ StructMappedDataType, StructFieldVisitor },
    builder::literal,
  },
  test::util,
};

#[derive(Clone, Copy)]
struct Uniforms {
  dims: [u32; 2],
}
impl StructMappedDataType for Uniforms {
  const NAME: &'static str = "Uniforms";
  fn visit_fields<FV>(fv: &mut FV)
    where FV: StructFieldVisitor<Uniforms>
  {
    fv.visit_field::<[u32; 2], _, _>("dims", |u| u.dims, |u, v| u.dims = v);
  }
}

#[test]
fn smoketest_perlin() {
  let (device, queue) = util::get_device_and_queue();
  let project = Project::new(device, queue);
  let shader = project.define_shader::<Uniforms, _>(|shb| {

    //////
    // FUNCTION: rot_left(val: vec4<u32>, rot: vec4<u32>) -> vec4<u32>
    shb.define_function::<
      /* args */ ([u32; 4], [u32; 4]),
      /* ret */ [u32; 4],
      _,
    > (
      "rot_left", ("val", "rot"),
      |cbb, (val, rot)| {
        cbb.add_return_statement(
          (val.clone() << rot.clone()) |
          (val >> (literal(32) - rot))
        );
      }
    );

    shb.define_constant("XXHASH_PRIME_1", 0x9E3779B1_u32);
    shb.define_constant("XXHASH_PRIME_2", 0x85EBCA77_u32);
    shb.define_constant("XXHASH_PRIME_3", 0xC2B2AE3D_u32);

    //////
    // FUNCTION: xxhash32(seed: u32, values: vec4<u32>) -> u32
    shb.define_function::<
      /* args */ (u32, [u32; 4]),
      /* ret */ u32,
      _,
    > (
      "xxhash32", ("seed", "values"),
      |cbb, (seed, values)| {
      }
    );
  });
  eprintln!("shader: {:?}", &shader);
  eprintln!("");
  eprintln!("");
  let wgsl_code = shader.generate_wgsl();
  eprintln!("wgsl_code:");
  eprintln!("{}", &wgsl_code);
}
