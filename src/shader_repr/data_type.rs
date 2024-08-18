use crate::shader_repr::{
  native_type::ShaderNativeTypeRepr,
  structure::ShaderStructureRepr,
};

pub(crate) enum ShaderDataTypeRepr {
  Native(ShaderNativeTypeRepr),
  Structure(ShaderStructureRepr),
}

pub(crate) enum ShaderDataTypeUseRepr {
  Native(ShaderNativeTypeRepr),
  Structure(String),
}

pub(crate) enum ShaderReturnDataTypeUseRepr {
  Void,
  Boolean,
  Native(ShaderNativeTypeRepr),
  Structure(String),
}
