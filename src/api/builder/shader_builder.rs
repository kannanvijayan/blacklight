use crate::{
  api::{
    builder::{ CodeBlockBuilder, ExprHandle },
    data_type::ShEpArgDataType,
    EntryPoint,
    Project,
    Shader,
  },
  model::{ EntryPointModel, ExpressionModel, ShaderModel },
};

/**
 * A builder helper for defining shaders.
 */
pub struct ShaderBuilder<'pr> {
  _project: &'pr Project,
  entrypoints: Vec<EntryPointModel>,
}
impl<'pr> ShaderBuilder<'pr> {
  /** Create a new shader builder for the given project builder. */
  pub(crate) fn new(project: &'pr Project) -> Self {
    ShaderBuilder { _project: project, entrypoints: Vec::new() }
  }

  /** Define a new linear shader entrypoint. */
  pub fn define_entrypoint<'sh, ARG, EPB>(&'sh mut self,
    name: &'static str,
    builder_func: EPB
  ) -> EntryPoint<ARG>
  where
    EPB: FnOnce(&mut CodeBlockBuilder<'sh, 'sh, ()>, ExprHandle<'sh, ARG>),
    ARG: ShEpArgDataType
  {
    let mut code_block_builder = CodeBlockBuilder::new();
    let arg_expr = ExprHandle::new(ExpressionModel::identifier("global_id"));
    builder_func(&mut code_block_builder, arg_expr);
    let code_block_model = code_block_builder.build();
    let entry_point_model = EntryPointModel::new(name.into(), code_block_model);
    self.add_entry_point(entry_point_model.clone());
    EntryPoint::new(entry_point_model)
  }

  /** Build the shader from the definitions provided. */
  pub(crate) fn build(self) -> Shader {
    let shader_model = ShaderModel::new(self.entrypoints);
    Shader::new(shader_model)
  }

  /** Add an entrypoint. */
  pub(crate) fn add_entry_point(&mut self, entrypoint: EntryPointModel) {
    self.entrypoints.push(entrypoint);
  }
}
