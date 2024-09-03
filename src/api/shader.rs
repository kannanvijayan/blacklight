use crate::{
  model::ShaderModel,
  printer::generate_wgsl,
};

/**
 * A self-contained representation of a shader.
 */
#[derive(Clone, Debug)]
pub struct Shader {
  model: ShaderModel,
}
impl Shader {
  /** Create a new empty shader. */
  pub(crate) fn new(
    model: ShaderModel,
  ) -> Self {
    Shader { model }
  }

  /** Generate the wgsl for this shader. */
  pub fn generate_wgsl(&self) -> String {
    generate_wgsl(&self.model)
  }
}
