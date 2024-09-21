use crate::{
  api::{
    Project,
    data_type::{
      StructMappedDataType,
      StructFieldVisitor,
    },
    builder::{ literal, mkvec },

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
    let rot_left = shb.define_function::<
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

    let xxhash_prime_1 =
      shb.define_constant("XXHASH_PRIME_1", 0x9E3779B1_u32);
    let xxhash_prime_2 =
      shb.define_constant("XXHASH_PRIME_2", 0x85EBCA77_u32);
    let _xxhash_prime_3 = shb.define_constant("XXHASH_PRIME_3", 0xC2B2AE3D_u32);

    //////
    // FUNCTION: xxhash32(seed: u32, values: vec4<u32>) -> u32
    shb.define_function::<
      /* args */ (u32, [u32; 4]),
      /* ret */ u32,
      _,
    > (
      "xxhash32", ("seed", "values"),
      |cbb, (seed, values)| {
        let state = cbb.add_let_decl_statement("state",
          mkvec::<[u32; 4], _>((
            seed.clone() + xxhash_prime_1.read() + xxhash_prime_2.read(),
            seed.clone() + xxhash_prime_2.read(),
            seed.clone(),
            seed - xxhash_prime_1.read(),
          ))
        );
        let pre_rotate = cbb.add_let_decl_statement("pre_rotate",
          (state.read() + values) * xxhash_prime_2.read()
        );
        let _new_state = cbb.add_let_decl_statement("new_state",
          rot_left.call((
            rot_left.call((
              pre_rotate.read(),
              mkvec::<[u32; 4], _>(13_u32)
            )) * xxhash_prime_1.read(),
            mkvec::<[u32; 4], _>((1_u32, 7_u32, 12_u32, 18_u32))
          ))
        );
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
