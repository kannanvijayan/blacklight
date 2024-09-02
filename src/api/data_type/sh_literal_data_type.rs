use crate::api::data_type::ShExprDataType;


/**
 * Data types that can have literal values in the shader source.
 */
pub trait ShLiteralDataType: 'static + Sized + ShExprDataType {
  /** Convert to an `ShLiteralDataValue` */
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue;
}

impl ShLiteralDataType for bool {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Bool(*self)
  }
}

impl ShLiteralDataType for i32 {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::I32(*self)
  }
}
impl ShLiteralDataType for [i32; 2] {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Vec2I32(*self)
  }
}
impl ShLiteralDataType for [i32; 3] {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Vec3I32(*self)
  }
}
impl ShLiteralDataType for [i32; 4] {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Vec4I32(*self)
  }
}

impl ShLiteralDataType for u32 {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::U32(*self)
  }
}
impl ShLiteralDataType for [u32; 2] {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Vec2U32(*self)
  }
}
impl ShLiteralDataType for [u32; 3] {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Vec3U32(*self)
  }
}
impl ShLiteralDataType for [u32; 4] {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Vec4U32(*self)
  }
}

impl ShLiteralDataType for f32 {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::F32(*self)
  }
}
impl ShLiteralDataType for [f32; 2] {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Vec2F32(*self)
  }
}
impl ShLiteralDataType for [f32; 3] {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Vec3F32(*self)
  }
}
impl ShLiteralDataType for [f32; 4] {
  fn to_sh_literal_data_value(&self) -> ShLiteralDataValue {
    ShLiteralDataValue::Vec4F32(*self)
  }
}

/**
 * A type erasure from static and incorporation into runtime for a literal data type.
 */
#[derive(Clone, Debug)]
pub enum ShLiteralDataValue {
  Bool(bool),
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
