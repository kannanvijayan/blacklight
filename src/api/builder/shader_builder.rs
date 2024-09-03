use std::{collections::HashSet, marker::PhantomData};
use crate::{
  api::{
    builder::CodeBlockBuilder,
    data_type::{ BufferDataType, EntryPointArgDataType },
    handle::{ BufferBindingHandle, ExprHandle },
    EntryPoint,
    Project,
    Shader,
  },
  model::{
    BufferBindingModel,
    EntryPointModel,
    ExpressionModel,
    IdentifierExprModel,
    ShaderModel,
  },
};

/**
 * A builder helper for defining shaders.
 */
pub struct ShaderBuilder<'sh, 'pr: 'sh> {
  _project: &'pr Project,
  entrypoints: Vec<EntryPointModel>,
  buffer_bindings: Vec<BufferBindingModel>,
  used_buffer_bindings: HashSet<(u32, u32)>,
  _phantom: PhantomData<& 'sh ()>
}
impl<'sh, 'pr: 'sh> ShaderBuilder<'sh, 'pr> {
  /** Create a new shader builder for the given project builder. */
  pub(crate) fn new(project: &'pr Project) -> Self {
    ShaderBuilder {
      _project: project,
      entrypoints: Vec::new(),
      buffer_bindings: Vec::new(),
      used_buffer_bindings: HashSet::new(),
      _phantom: PhantomData,
    }
  }

  /** Define a new linear shader entrypoint. */
  pub fn define_entrypoint<ARG, EPB>(&mut self,
    name: &'static str,
    builder_func: EPB
  ) -> EntryPoint<ARG>
  where
    EPB: FnOnce(&mut CodeBlockBuilder<'sh, 'sh, ()>, ExprHandle<'sh, ARG>),
    ARG: EntryPointArgDataType
  {
    let mut code_block_builder = CodeBlockBuilder::new();
    let ident_expr_model = IdentifierExprModel::new("global_id".to_string());
    let arg_expr =
      ExprHandle::new(ExpressionModel::Identifier(ident_expr_model));
    builder_func(&mut code_block_builder, arg_expr);
    let code_block_model = code_block_builder.build();
    let entry_point_model = EntryPointModel::new(name.into(), code_block_model);
    self.entrypoints.push(entry_point_model.clone());
    EntryPoint::new(entry_point_model)
  }

  /** Define a buffer binding. */
  pub fn define_buffer_binding<DT>(&mut self,
    name: &str,
    group: u32,
    index: u32
  ) -> BufferBindingHandle<'sh, DT>
  where DT: BufferDataType
  {
    if self.used_buffer_bindings.contains(&(group, index)) {
      panic!("Buffer binding with group {} and index {} already defined.", group, index);
    }
    let buffer_binding_model = BufferBindingModel::new(
      name.to_string(),
      group,
      index,
      DT::REPR
    );
    self.buffer_bindings.push(buffer_binding_model);
    self.used_buffer_bindings.insert((group, index));
    BufferBindingHandle::new(name.to_string())
  }

  /** Build the shader from the definitions provided. */
  pub(crate) fn build(self) -> Shader {
    let shader_model =
      ShaderModel::new(self.buffer_bindings, self.entrypoints);
    Shader::new(shader_model)
  }
}
