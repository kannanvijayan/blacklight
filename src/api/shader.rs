use std::{
  fmt,
  marker::PhantomData,
};
use crate::{
  api::data_type::StructMappedDataType,
  model::ShaderModel,
  printer::generate_wgsl,
};

/**
 * A self-contained representation of a shader.
 */
#[derive(Clone)]
pub struct Shader<UDT>
  where UDT: StructMappedDataType
{
  model: ShaderModel,
  _phantom: PhantomData<UDT>
}
impl<UDT> Shader<UDT>
  where UDT: StructMappedDataType
{
  /** Create a new empty shader. */
  pub(crate) fn new(
    model: ShaderModel,
  ) -> Self {
    Shader { model, _phantom: PhantomData }
  }

  /** Generate the wgsl for this shader. */
  pub fn generate_wgsl(&self) -> String {
    generate_wgsl(&self.model)
  }
}
impl<UDT> fmt::Debug for Shader<UDT>
  where UDT: StructMappedDataType
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Shader")
      .field("model", &self.model)
      .finish()
  }
}
