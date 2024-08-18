use crate::shader_repr::{
  workgroup_size::ShaderWorkgroupSizeRepr,
  body::ShaderBodyRepr,
};

pub(crate) struct ShaderEntryPointRepr {
  name: String,
  workgroup_size: ShaderWorkgroupSizeRepr,
  body: ShaderBodyRepr,
}
