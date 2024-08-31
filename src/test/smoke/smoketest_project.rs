use crate::{
  api::Project,
  test::util,
};

#[test]
fn smoketest_project() {
  let (device, queue) = util::get_device_and_queue();
  let project = Project::new(device, queue);
  let shader = project.define_shader(|bsh| {
    bsh.define_entrypoint::<u32, _>("main", |cbb, id| {
      cbb.add_expr_statement(id.clone());
      let var_foo = cbb.add_var_decl_statement("foo", id.clone());
      cbb.add_assignment_statement(var_foo, id);
      cbb.add_bare_return_statement();
    });
  });
  eprintln!("shader: {:?}", &shader);
}
