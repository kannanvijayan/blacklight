use crate::{
  api::{
    Project,
    data_type::{ Struct, StructMappedDataType, StructFieldVisitor },
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
    where FV: StructFieldVisitor
  {
    fv.visit_field::<u32>("x");
    fv.visit_field::<u32>("y");
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
    where FV: StructFieldVisitor
  {
    fv.visit_field::<Struct<Point>>("top_left");
    fv.visit_field::<Struct<Point>>("bottom_right");
  }
}

#[test]
fn smoketest_project() {
  let (device, queue) = util::get_device_and_queue();
  let project = Project::new(device, queue);
  let shader = project.define_shader(|shb| {
    let ints_buf = shb.define_read_buffer_binding::<u32>("ints", 0, 0);
    let rects_buf = shb.define_read_buffer_binding::<Struct<Rect>>("rects", 0, 1);
    shb.define_entrypoint::<u32, _>("main", 64, |cbb, id| {
      cbb.add_expr_statement(id.clone());
      let var_foo = cbb.add_var_decl_statement("foo", id.clone());
      cbb.add_if_else_statement(
        var_foo.read().eq(&cbb.literal_expr(33)),
        |cbb| {
          cbb.add_assignment_statement(&var_foo, id.clone());
          cbb.add_assignment_statement(
            &ints_buf.elem(cbb.literal_expr(0)),
            var_foo.read() + cbb.literal_expr(1)
          );
        },
        |cbb| {
          cbb.add_assignment_statement(&var_foo, var_foo.read());
        }
      );
      cbb.add_bare_return_statement();
    });
  });
  eprintln!("shader: {:?}", &shader);
  eprintln!("");
  eprintln!("");
  let wgsl_code = shader.generate_wgsl();
  eprintln!("wgsl_code:");
  eprintln!("{}", &wgsl_code);
}
