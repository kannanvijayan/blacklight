use crate::shader_repr::data_type::ShaderDataTypeRepr;

pub(crate) struct ShaderBindingRepr {
  name: String,
  binding: u32,
  data_type: ShaderDataTypeRepr,
}
