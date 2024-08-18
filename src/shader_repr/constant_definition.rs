use crate::shader_repr::native_value::ShaderNativeValueRepr;

pub(crate) struct ShaderConstantDefinitionRepr {
  name: String,
  value: ShaderNativeValueRepr,
}
