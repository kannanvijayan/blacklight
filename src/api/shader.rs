use crate::model::ShaderModel;

/**
 * A self-contained representation of a shader.
 */
#[derive(Clone, Debug)]
pub struct Shader {
  _model: ShaderModel,
}
impl Shader {
  /** Create a new empty shader. */
  pub(crate) fn new(model: ShaderModel) -> Self {
    Shader { _model: model }
  }
}
