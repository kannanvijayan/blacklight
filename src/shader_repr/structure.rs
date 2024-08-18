use crate::shader_repr::data_type::ShaderDataTypeUseRepr;

pub(crate) struct ShaderStructureRepr {
  pub(crate) name: String,
  pub(crate) fields: Vec<ShaderStructureFieldRepr>,
}

pub(crate) struct ShaderStructureFieldRepr {
  pub(crate) name: String,
  pub(crate) data_type: ShaderDataTypeUseRepr,
}
