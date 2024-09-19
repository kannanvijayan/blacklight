use crate::{
  api::{
    Project,
    data_type::{ Struct, StructMappedDataType, StructFieldVisitor },
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
fn smoketest_project() {
  let (device, queue) = util::get_device_and_queue();
  let project = Project::new(device, queue);
  let shader = project.define_shader::<Uniforms, _>(|shb| {
    // Define some buffer bindings.
    let ints_buf = shb.define_read_write_buffer_binding::<u32>("ints", 0, 0);
    let rects_buf = shb.define_read_buffer_binding::<Struct<Rect>>("rects", 0, 1);

    // Define a function.
    let foo_func = shb.define_function::<(u32,), u32, _>("foofunc", ("x",), |cbb, args| {
      cbb.add_return_statement(args.0 + literal(33));
    });

    // Define an entrypoint.
    shb.define_entrypoint::<u32, _>("main", 64, |cbb, id| {
      cbb.add_expr_statement(id.clone());
      let var_foo = cbb.add_var_decl_statement("varfoo", id.clone());
      let var_rect = rects_buf.read(literal(0));
      cbb.add_if_else_statement(
        var_foo.read().eq(&literal(33)),
        |cbb| {
          cbb.add_assignment_statement(&var_foo.lvalue(), id.clone());
          cbb.add_assignment_statement(
            &ints_buf.elem(literal(0)),
            var_foo.read() + literal(1)
          );
          cbb.add_assignment_statement(
            &ints_buf.elem(literal(1)),
            foo_func.call((var_foo.read(),))
          );
        },
        |cbb| {
          cbb.add_assignment_statement(&var_foo.lvalue(), var_foo.read());
          cbb.add_assignment_statement(
            &var_rect
              .get::<Struct<Point>>("top_left")
              .field::<u32>("x"),
            id.clone());
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
