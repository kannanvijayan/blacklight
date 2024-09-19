use crate::api::data_type::{ DataTypeRepr, ExprDataType };

/**
 * Data types that can have literal values in the shader source.
 */
pub trait LiteralDataType: ExprDataType {
  /** Convert to an `ShLiteralDataValue` */
  fn to_literal_data_value(&self) -> LiteralDataValue;
}

impl LiteralDataType for bool {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Bool(*self)
  }
}

impl LiteralDataType for i32 {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::I32(*self)
  }
}
impl LiteralDataType for [i32; 2] {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec2I32(*self)
  }
}
impl LiteralDataType for [i32; 3] {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec3I32(*self)
  }
}
impl LiteralDataType for [i32; 4] {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec4I32(*self)
  }
}

impl LiteralDataType for u32 {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::U32(*self)
  }
}
impl LiteralDataType for [u32; 2] {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec2U32(*self)
  }
}
impl LiteralDataType for [u32; 3] {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec3U32(*self)
  }
}
impl LiteralDataType for [u32; 4] {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec4U32(*self)
  }
}

impl LiteralDataType for f32 {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::F32(*self)
  }
}
impl LiteralDataType for [f32; 2] {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec2F32(*self)
  }
}
impl LiteralDataType for [f32; 3] {
  fn to_literal_data_value(&self) -> LiteralDataValue {
    LiteralDataValue::Vec3F32(*self)
  }
}
impl LiteralDataType for [f32; 4] {
  fn to_literal_data_value(&self) -> LiteralDataValue {
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
impl LiteralDataValue {
  /** Get the datatype for this value. */
  pub fn data_type_repr(&self) -> DataTypeRepr {
    match self {
      LiteralDataValue::Bool(_) => DataTypeRepr::new_bool(),
      LiteralDataValue::I32(_) => DataTypeRepr::new_i32(),
      LiteralDataValue::Vec2I32(_) => DataTypeRepr::new_vec2_i32(),
      LiteralDataValue::Vec3I32(_) => DataTypeRepr::new_vec3_i32(),
      LiteralDataValue::Vec4I32(_) => DataTypeRepr::new_vec4_i32(),
      LiteralDataValue::U32(_) => DataTypeRepr::new_u32(),
      LiteralDataValue::Vec2U32(_) => DataTypeRepr::new_vec2_u32(),
      LiteralDataValue::Vec3U32(_) => DataTypeRepr::new_vec3_u32(),
      LiteralDataValue::Vec4U32(_) => DataTypeRepr::new_vec4_u32(),
      LiteralDataValue::F32(_) => DataTypeRepr::new_f32(),
      LiteralDataValue::Vec2F32(_) => DataTypeRepr::new_vec2_f32(),
      LiteralDataValue::Vec3F32(_) => DataTypeRepr::new_vec3_f32(),
      LiteralDataValue::Vec4F32(_) => DataTypeRepr::new_vec4_f32(),
    }
  }

  /** Generate a wgsl source string for this literal. */
  pub fn wgsl_source(&self) -> String {
    match self {
      LiteralDataValue::Bool(b) => {
        (if *b { "true" } else { "false" }).to_string()
      },
      LiteralDataValue::I32(int32) => {
        format!("{}i", int32)
      },
      LiteralDataValue::Vec2I32([x, y]) => {
        format!("vec2<i32>({}, {})", x, y)
      },
      LiteralDataValue::Vec3I32([x, y, z]) => {
        format!("vec3<i32>({}, {}, {})", x, y, z)
      },
      LiteralDataValue::Vec4I32([x, y, z, w]) => {
        format!("vec4<i32>({}, {}, {}, {})", x, y, z, w)
      },
      LiteralDataValue::U32(uint32) => {
        format!("{}u", uint32)
      },
      LiteralDataValue::Vec2U32([x, y]) => {
        format!("vec2<u32>({}, {})", x, y)
      },
      LiteralDataValue::Vec3U32([x, y, z]) => {
        format!("vec3<u32>({}, {}, {})", x, y, z)
      },
      LiteralDataValue::Vec4U32([x, y, z, w]) => {
        format!("vec4<u32>({}, {}, {}, {})", x, y, z, w)
      },
      LiteralDataValue::F32(float32) => {
        format!("{}f", float32)
      },
      LiteralDataValue::Vec2F32([x, y]) => {
        format!("vec2<f32>({}, {})", x, y)
      },
      LiteralDataValue::Vec3F32([x, y, z]) => {
        format!("vec3<f32>({}, {}, {})", x, y, z)
      },
      LiteralDataValue::Vec4F32([x, y, z, w]) => {
        format!("vec4<f32>({}, {}, {}, {})", x, y, z, w)
      },
    }
  }
}
