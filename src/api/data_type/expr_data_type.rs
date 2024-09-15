use crate::api::data_type::{ DataTypeRepr, Struct, StructMappedDataType };

/**
 * Data types that can be the result of expressions.
 */
pub trait ExprDataType: 'static + Sized + Clone + Copy {
  /** The runtime representation of the type. */
  fn repr() -> DataTypeRepr;
}

impl ExprDataType for bool {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_bool()
  }
}

impl ExprDataType for i32 {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_i32()
  }
}
impl ExprDataType for [i32; 2] {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_vec2_i32()
  }
}
impl ExprDataType for [i32; 3] {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_vec3_i32()
  }
}
impl ExprDataType for [i32; 4] {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_vec4_i32()
  }
}

impl ExprDataType for u32 {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_u32()
  }
}
impl ExprDataType for [u32; 2] {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_vec2_u32()
  }
}
impl ExprDataType for [u32; 3] {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_vec3_u32()
  }
}
impl ExprDataType for [u32; 4] {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_vec4_u32()
  }
}

impl ExprDataType for f32 {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_f32()
  }
}
impl ExprDataType for [f32; 2] {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_vec2_f32()
  }
}
impl ExprDataType for [f32; 3] {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_vec3_f32()
  }
}
impl ExprDataType for [f32; 4] {
  fn repr() -> DataTypeRepr {
    DataTypeRepr::new_vec4_f32()
  }
}

impl<T> ExprDataType for Struct<T>
  where T: Copy + StructMappedDataType
{
  fn repr() -> DataTypeRepr {
    Struct::<T>::make_repr()
  }
}


/**
 * Expr data types that are numeric in nature (i.e. are field data types).
 */
pub trait ExprNumericDataType: ExprDataType {
}

impl ExprNumericDataType for i32 {}
impl ExprNumericDataType for [i32; 2] {}
impl ExprNumericDataType for [i32; 3] {}
impl ExprNumericDataType for [i32; 4] {}

impl ExprNumericDataType for u32 {}
impl ExprNumericDataType for [u32; 2] {}
impl ExprNumericDataType for [u32; 3] {}
impl ExprNumericDataType for [u32; 4] {}

impl ExprNumericDataType for f32 {}
impl ExprNumericDataType for [f32; 2] {}
impl ExprNumericDataType for [f32; 3] {}
impl ExprNumericDataType for [f32; 4] {}
