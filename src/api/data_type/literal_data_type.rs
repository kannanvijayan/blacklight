use crate::api::data_type::ExprDataType;


/**
 * Data types that can have literal values in the shader source.
 */
pub trait LiteralDataType: 'static + Sized + ExprDataType {
  /** Convert to an `ShLiteralDataValue` */
  fn to_sh_literal_data_value(&self) -> LiteralDataValue;
}

impl LiteralDataType for bool {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Bool(*self)
  }
}

impl LiteralDataType for i32 {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::I32(*self)
  }
}
impl LiteralDataType for [i32; 2] {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec2I32(*self)
  }
}
impl LiteralDataType for [i32; 3] {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec3I32(*self)
  }
}
impl LiteralDataType for [i32; 4] {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec4I32(*self)
  }
}

impl LiteralDataType for u32 {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::U32(*self)
  }
}
impl LiteralDataType for [u32; 2] {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec2U32(*self)
  }
}
impl LiteralDataType for [u32; 3] {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec3U32(*self)
  }
}
impl LiteralDataType for [u32; 4] {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec4U32(*self)
  }
}

impl LiteralDataType for f32 {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::F32(*self)
  }
}
impl LiteralDataType for [f32; 2] {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec2F32(*self)
  }
}
impl LiteralDataType for [f32; 3] {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec3F32(*self)
  }
}
impl LiteralDataType for [f32; 4] {
  fn to_sh_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec4F32(*self)
  }
}

/**
 * A type erasure from static and incorporation into runtime for a literal data type.
 */
#[derive(Clone, Debug)]
pub enum LiteralDataValue {
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
