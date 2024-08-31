use crate::model::EntryPointModel;

/**
 * Model of a shader.
 */
#[derive(Clone, Debug)]
pub(crate) struct ShaderModel {
  _entrypoints: Vec<EntryPointModel>,
}
impl ShaderModel {
  /** Create a new shader model. */
  pub(crate) fn new(entrypoints: Vec<EntryPointModel>) -> Self {
    ShaderModel { _entrypoints: entrypoints }
  }
}
