use crate::model::{
  EntryPointModel,
  BufferBindingModel,
};

/**
 * Model of a shader.
 */
#[derive(Clone, Debug)]
pub(crate) struct ShaderModel {
  buffer_bindings: Vec<BufferBindingModel>,
  entrypoints: Vec<EntryPointModel>,
}
impl ShaderModel {
  /** Create a new shader model. */
  pub(crate) fn new(
    buffer_bindings: Vec<BufferBindingModel>,
    entrypoints: Vec<EntryPointModel>,
  ) -> Self {
    ShaderModel {
      buffer_bindings,
      entrypoints,
    }
  }

  /** Get the buffer bindings. */
  pub(crate) fn buffer_bindings(&self) -> &[BufferBindingModel] {
    &self.buffer_bindings
  }

  /** Get the entrypoints. */
  pub(crate) fn entrypoints(&self) -> &[EntryPointModel] {
    &self.entrypoints
  }
}
