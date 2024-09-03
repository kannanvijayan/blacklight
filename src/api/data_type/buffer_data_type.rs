use crate::api::data_type::ExprDataType;

/**
 * Data types that can have literal values in the shader source.
 */
pub trait BufferDataType: 'static + Sized + ExprDataType {
  /** The runtime representation of the type. */
  const REPR: BufferDataTypeRepr;

  /** Convert to an `ShBufferDataValue` */
  fn to_sh_literal_data_value(&self) -> BufferDataValue;
}

impl BufferDataType for i32 {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::I32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::I32(*self)
  }
}
impl BufferDataType for [i32; 2] {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::Vec2I32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::Vec2I32(*self)
  }
}
impl BufferDataType for [i32; 3] {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::Vec3I32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::Vec3I32(*self)
  }
}
impl BufferDataType for [i32; 4] {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::Vec4I32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::Vec4I32(*self)
  }
}

impl BufferDataType for u32 {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::U32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::U32(*self)
  }
}
impl BufferDataType for [u32; 2] {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::Vec2U32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::Vec2U32(*self)
  }
}
impl BufferDataType for [u32; 3] {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::Vec3U32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::Vec3U32(*self)
  }
}
impl BufferDataType for [u32; 4] {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::Vec4U32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::Vec4U32(*self)
  }
}

impl BufferDataType for f32 {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::F32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::F32(*self)
  }
}
impl BufferDataType for [f32; 2] {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::Vec2F32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::Vec2F32(*self)
  }
}
impl BufferDataType for [f32; 3] {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::Vec3F32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::Vec3F32(*self)
  }
}
impl BufferDataType for [f32; 4] {
  const REPR: BufferDataTypeRepr = BufferDataTypeRepr::Vec4F32;
  fn to_sh_literal_data_value(&self) -> BufferDataValue {
    BufferDataValue::Vec4F32(*self)
  }
}

#[derive(Clone, Debug)]
pub enum BufferDataTypeRepr {
  I32, Vec2I32, Vec3I32, Vec4I32,
  U32, Vec2U32, Vec3U32, Vec4U32,
  F32, Vec2F32, Vec3F32, Vec4F32,
}

/**
 * A type erasure from static and incorporation into runtime for a literal data type.
 */
#[derive(Clone, Debug)]
pub enum BufferDataValue {
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
