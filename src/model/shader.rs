use crate::{
  api::data_type::StructDataTypeRepr,
  model::{
    BufferBindingModel,
    EntryPointModel,
    FunctionModel,
  },
};

/**
 * Model of a shader.
 */
#[derive(Clone, Debug)]
pub(crate) struct ShaderModel {
  struct_data_types: Vec<StructDataTypeRepr>,
  buffer_bindings: Vec<BufferBindingModel>,
  functions: Vec<FunctionModel>,
  entrypoints: Vec<EntryPointModel>,
}
impl ShaderModel {
  /** Create a new shader model. */
  pub(crate) fn new(
    struct_data_types: Vec<StructDataTypeRepr>,
    buffer_bindings: Vec<BufferBindingModel>,
    functions: Vec<FunctionModel>,
    entrypoints: Vec<EntryPointModel>,
  ) -> Self {
    ShaderModel { struct_data_types, buffer_bindings, functions, entrypoints }
  }

  /** Get the struct data types. */
  pub(crate) fn struct_data_types(&self) -> &[StructDataTypeRepr] {
    &self.struct_data_types
  }

  /** Get the buffer bindings. */
  pub(crate) fn buffer_bindings(&self) -> &[BufferBindingModel] {
    &self.buffer_bindings
  }

  /** Get the functions */
  pub(crate) fn functions(&self) -> &[FunctionModel] {
    &self.functions
  }

  /** Get the entrypoints. */
  pub(crate) fn entrypoints(&self) -> &[EntryPointModel] {
    &self.entrypoints
  }
}
