use crate::model::IdentifierModel;

/**
 * Representation of a data type.
 */
#[derive(Clone, Debug)]
pub enum DataTypeRepr {
  Builtin(BuiltinDataTypeRepr),
  Struct(StructDataTypeRepr),
}
impl DataTypeRepr {
  /** Create a new data type representation. */
  pub fn new_builtin(builtin: BuiltinDataTypeRepr) -> Self {
    DataTypeRepr::Builtin(builtin)
  }

  // Helpers for creating built-in data types.
  pub fn new_void() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Void)
  }
  pub fn new_bool() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Bool)
  }
  pub fn new_i32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::I32)
  }
  pub fn new_vec2_i32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Vec2I32)
  }
  pub fn new_vec3_i32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Vec3I32)
  }
  pub fn new_vec4_i32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Vec4I32)
  }
  pub fn new_u32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::U32)
  }
  pub fn new_vec2_u32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Vec2U32)
  }
  pub fn new_vec3_u32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Vec3U32)
  }
  pub fn new_vec4_u32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Vec4U32)
  }
  pub fn new_f32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::F32)
  }
  pub fn new_vec2_f32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Vec2F32)
  }
  pub fn new_vec3_f32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Vec3F32)
  }
  pub fn new_vec4_f32() -> Self {
    DataTypeRepr::new_builtin(BuiltinDataTypeRepr::Vec4F32)
  }

  /** Create a new data type representation. */
  pub(crate) fn new_struct(name: IdentifierModel, fields: Vec<StructFieldRepr>) -> Self {
    DataTypeRepr::Struct(StructDataTypeRepr::new(name, fields))
  }

  /** Get a wgsl source string representation of this type. */
  pub(crate) fn wgsl_source<'a>(&'a self) -> &'a str {
    match self {
      DataTypeRepr::Builtin(builtin) => match builtin {
        BuiltinDataTypeRepr::Void => "void",
        BuiltinDataTypeRepr::Bool => "bool",
        BuiltinDataTypeRepr::I32 => "i32",
        BuiltinDataTypeRepr::Vec2I32 => "vec2<i32>",
        BuiltinDataTypeRepr::Vec3I32 => "vec3<i32>",
        BuiltinDataTypeRepr::Vec4I32 => "vec4<i32>",
        BuiltinDataTypeRepr::U32 => "u32",
        BuiltinDataTypeRepr::Vec2U32 => "vec2<u32>",
        BuiltinDataTypeRepr::Vec3U32 => "vec3<u32>",
        BuiltinDataTypeRepr::Vec4U32 => "vec4<u32>",
        BuiltinDataTypeRepr::F32 => "f32",
        BuiltinDataTypeRepr::Vec2F32 => "vec2<f32>",
        BuiltinDataTypeRepr::Vec3F32 => "vec3<f32>",
        BuiltinDataTypeRepr::Vec4F32 => "vec4<f32>",
      },
      DataTypeRepr::Struct(struct_data_type) => struct_data_type.name(),
    }
  }

  pub(crate) fn get_struct(&self) -> Option<&StructDataTypeRepr> {
    match self {
      DataTypeRepr::Struct(struct_data_type) => Some(struct_data_type),
      _ => None,
    }
  }
}

/**
 * Model of a built-in data type.
 * This covers all possible built-in data types in WGSL, regardless of
 * what context they are allowed to be used in.
 */
#[derive(Clone, Debug)]
pub enum BuiltinDataTypeRepr {
  Void, Bool,
  I32, Vec2I32, Vec3I32, Vec4I32,
  U32, Vec2U32, Vec3U32, Vec4U32,
  F32, Vec2F32, Vec3F32, Vec4F32,
}

/**
 * Representation of a struct definition.
 */
#[derive(Clone, Debug)]
pub struct StructDataTypeRepr {
  name: IdentifierModel,
  fields: Vec<StructFieldRepr>,
}
impl StructDataTypeRepr {
  /** Create a new struct data type representation. */
  pub fn new(name: IdentifierModel, fields: Vec<StructFieldRepr>) -> Self {
    StructDataTypeRepr { name, fields }
  }

  /** Get the name of the struct. */
  pub fn name(&self) -> &str {
    &self.name.as_str()
  }

  /** Get the fields of the struct. */
  pub fn fields(&self) -> &[StructFieldRepr] {
    &self.fields
  }
}

/**
 * Representation of a struct field.
 */
#[derive(Clone, Debug)]
pub struct StructFieldRepr {
  name: IdentifierModel,
  data_type: DataTypeRepr,
}
impl StructFieldRepr {
  /** Create a new struct field representation. */
  pub fn new(name: IdentifierModel, data_type: DataTypeRepr) -> Self {
    StructFieldRepr { name, data_type }
  }

  /** Get the name of the field. */
  pub fn name(&self) -> &str {
    &self.name.as_str()
  }

  /** Get the type of the field. */
  pub fn data_type(&self) -> &DataTypeRepr {
    &self.data_type
  }
}
