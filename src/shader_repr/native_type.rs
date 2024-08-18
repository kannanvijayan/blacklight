
pub(crate) enum ShaderNativeTypeRepr {
  Scalar(ShaderNativeScalarTypeRepr),
  Vec2(ShaderNativeScalarTypeRepr),
  Vec3(ShaderNativeScalarTypeRepr),
  Vec4(ShaderNativeScalarTypeRepr),
}

pub(crate) enum ShaderNativeScalarTypeRepr {
  I32,
  U32,
  F32,
}
