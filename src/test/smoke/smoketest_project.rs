use crate::{
  api::Project,
  test::util,
};

#[test]
fn smoketest_project() {
  let (device, queue) = util::get_device_and_queue();
  let project = Project::new(device, queue);
  let shader = project.define_shader(|shb| {
    let points_buf = shb.define_buffer_binding::<u32>("points", 0, 0);
    shb.define_entrypoint::<u32, _>("main", |cbb, id| {
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
}
