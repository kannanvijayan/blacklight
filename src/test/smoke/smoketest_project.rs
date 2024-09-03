use crate::{
  api::Project,
  printer::generate_wgsl,
  test::util,
};

#[test]
fn smoketest_project() {
  let (device, queue) = util::get_device_and_queue();
  let project = Project::new(device, queue);
  let shader = project.define_shader(|shb| {
    let points_buf = shb.define_read_buffer_binding::<u32>("points", 0, 0);
    shb.define_entrypoint::<u32, _>("main", 64, |cbb, id| {
      cbb.add_expr_statement(id.clone());
      let var_foo = cbb.add_var_decl_statement("foo", id.clone());
      cbb.add_if_else_statement(
        var_foo.read().eq(&cbb.literal_expr(33)),
        |cbb| {
          cbb.add_assignment_statement(&var_foo, id.clone());
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
