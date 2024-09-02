use crate::model::{
  EntryPointModel,
  BufferBindingModel,
};

/**
 * Model of a shader.
 */
#[derive(Clone, Debug)]
pub(crate) struct ShaderModel {
  _buffer_bindings: Vec<BufferBindingModel>,
  _entrypoints: Vec<EntryPointModel>,
}
impl ShaderModel {
  /** Create a new shader model. */
  pub(crate) fn new(
    buffer_bindings: Vec<BufferBindingModel>,
    entrypoints: Vec<EntryPointModel>,
  ) -> Self {
    ShaderModel {
      _buffer_bindings: buffer_bindings,
      _entrypoints: entrypoints,
    }
  }
}
