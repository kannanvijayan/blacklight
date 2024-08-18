use crate::shader_repr::data_type::ShaderDataTypeRepr;

pub(crate) struct ShaderTypeDefinitionRepr {
  name: String,
  fields: Vec<ShaderTypeDefinitionFieldRepr>,
}

pub(crate) struct ShaderTypeDefinitionFieldRepr {
  name: String,
  data_type: ShaderDataTypeRepr,
}
