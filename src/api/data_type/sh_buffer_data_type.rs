use crate::api::data_type::ShExprDataType;

/**
 * Data types that can have literal values in the shader source.
 */
pub trait ShBufferDataType: 'static + Sized + ShExprDataType {
  /** The runtime representation of the type. */
  const REPR: ShBufferDataTypeRepr;

  /** Convert to an `ShBufferDataValue` */
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue;
}

impl ShBufferDataType for i32 {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::I32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::I32(*self)
  }
}
impl ShBufferDataType for [i32; 2] {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::Vec2I32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::Vec2I32(*self)
  }
}
impl ShBufferDataType for [i32; 3] {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::Vec3I32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::Vec3I32(*self)
  }
}
impl ShBufferDataType for [i32; 4] {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::Vec4I32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::Vec4I32(*self)
  }
}

impl ShBufferDataType for u32 {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::U32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::U32(*self)
  }
}
impl ShBufferDataType for [u32; 2] {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::Vec2U32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::Vec2U32(*self)
  }
}
impl ShBufferDataType for [u32; 3] {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::Vec3U32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::Vec3U32(*self)
  }
}
impl ShBufferDataType for [u32; 4] {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::Vec4U32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::Vec4U32(*self)
  }
}

impl ShBufferDataType for f32 {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::F32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::F32(*self)
  }
}
impl ShBufferDataType for [f32; 2] {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::Vec2F32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::Vec2F32(*self)
  }
}
impl ShBufferDataType for [f32; 3] {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::Vec3F32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::Vec3F32(*self)
  }
}
impl ShBufferDataType for [f32; 4] {
  const REPR: ShBufferDataTypeRepr = ShBufferDataTypeRepr::Vec4F32;
  fn to_sh_literal_data_value(&self) -> ShBufferDataValue {
    ShBufferDataValue::Vec4F32(*self)
  }
}

#[derive(Clone, Debug)]
pub enum ShBufferDataTypeRepr {
  I32, Vec2I32, Vec3I32, Vec4I32,
  U32, Vec2U32, Vec3U32, Vec4U32,
  F32, Vec2F32, Vec3F32, Vec4F32,
}

/**
 * A type erasure from static and incorporation into runtime for a literal data type.
 */
#[derive(Clone, Debug)]
pub enum ShBufferDataValue {
  I32(i32),
  Vec2I32([i32; 2]),
  Vec3I32([i32; 3]),
  Vec4I32([i32; 4]),
  U32(u32),
  Vec2U32([u32; 2]),
  Vec3U32([u32; 3]),
  Vec4U32([u32; 4]),
  F32(f32),
  Vec2F32([f32; 2]),
  Vec3F32([f32; 3]),
  Vec4F32([f32; 4]),
}
