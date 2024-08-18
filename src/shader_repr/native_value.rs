
pub(crate) enum ShaderNativeValueRepr {
  Scalar(ShaderNativeScalarValueRepr),
  Vec2([ShaderNativeScalarValueRepr; 2]),
  Vec3([ShaderNativeScalarValueRepr; 3]),
  Vec4([ShaderNativeScalarValueRepr; 4]),
}

pub(crate) enum ShaderNativeScalarValueRepr {
  I32(i32),
  U32(u32),
  F32(f32),
}
