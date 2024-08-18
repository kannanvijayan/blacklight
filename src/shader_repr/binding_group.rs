use crate::shader_repr::binding::ShaderBindingRepr;

pub(crate) struct ShaderBindingGroupRepr {
  name: String,
  group: u32,
  bindings: Vec<ShaderBindingRepr>,
}
