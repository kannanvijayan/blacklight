use crate::{
  api::{
    Project,
    data_type::{ Struct, StructMappedDataType, StructFieldVisitor },
    builder::literal,
  },
  test::util,
};

#[derive(Clone, Copy)]
struct Point {
  x: u32,
  y: u32,
}
impl StructMappedDataType for Point {
  const NAME: &'static str = "Point";
  fn visit_fields<FV>(fv: &mut FV)
    where FV: StructFieldVisitor<Point>
  {
    fv.visit_field::<u32, _, _>("x", |p| p.x, |p, v| p.x = v);
    fv.visit_field::<u32, _, _>("y", |p| p.y, |p, v| p.y = v);
  }
}

#[derive(Clone, Copy)]
struct Rect {
  top_left: Point,
  bottom_right: Point,
}
impl StructMappedDataType for Rect {
  const NAME: &'static str = "Rect";
  fn visit_fields<FV>(fv: &mut FV)
    where FV: StructFieldVisitor<Rect>
  {
    fv.visit_field::<Struct<Point>, _, _>(
      "top_left",
      |r| r.top_left.into(),
      |r, v| r.top_left = *v.data()
    );
    fv.visit_field::<Struct<Point>, _, _>("bottom_right",
      |r| r.bottom_right.into(),
      |r, v| r.bottom_right = *v.data()
    );
  }
}

#[test]
fn smoketest_perlin() {
  let (device, queue) = util::get_device_and_queue();
  let project = Project::new(device, queue);
  let shader = project.define_shader(|shb| {
    shb.define_function::
      </* args */ ([u32; 4], [u32; 4]), /* ret */ [u32; 4], _>
    (
      "rot_left", ("val", "rot"),
      |cbb, args| {
        let val = args.0;
        let rot = args.1;
        cbb.add_return_statement(
          (val.clone() << rot.clone()) |
          (val >> (literal(32) - rot))
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
