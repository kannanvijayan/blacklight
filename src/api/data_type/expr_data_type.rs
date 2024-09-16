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
pub trait ExprNumericDataType: ExprDataType {}

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

/**
 * Expr data types that are scalar.
 */
pub trait ExprScalarDataType: ExprNumericDataType {
}

impl ExprScalarDataType for i32 {}
impl ExprScalarDataType for u32 {}
impl ExprScalarDataType for f32 {}

/**
 * Expr data types that are numeric in nature (i.e. are field data types).
 */
pub trait ExprVectorDataType: ExprNumericDataType {
  type Scalar: ExprScalarDataType;
}

impl ExprVectorDataType for [i32; 2] {
  type Scalar = i32;
}
impl ExprVectorDataType for [i32; 3] {
  type Scalar = i32;
}
impl ExprVectorDataType for [i32; 4] {
  type Scalar = i32;
}

impl ExprVectorDataType for [u32; 2] {
  type Scalar = u32;
}
impl ExprVectorDataType for [u32; 3] {
  type Scalar = u32;
}
impl ExprVectorDataType for [u32; 4] {
  type Scalar = u32;
}

impl ExprVectorDataType for [f32; 2] {
  type Scalar = f32;
}
impl ExprVectorDataType for [f32; 3] {
  type Scalar = f32;
}
impl ExprVectorDataType for [f32; 4] {
  type Scalar = f32;
}

/**
 * Expr data types that are numeric in nature (i.e. are field data types).
 */
pub trait ExprIntegralDataType: ExprNumericDataType {
  type AsUnsigned: ExprNumericDataType;
  type AsSigned: ExprNumericDataType;
}

impl ExprIntegralDataType for i32 {
  type AsUnsigned = u32;
  type AsSigned = i32;
}
impl ExprIntegralDataType for [i32; 2] {
  type AsUnsigned = [u32; 2];
  type AsSigned = [i32; 2];
}
impl ExprIntegralDataType for [i32; 3] {
  type AsUnsigned = [u32; 3];
  type AsSigned = [i32; 3];
}
impl ExprIntegralDataType for [i32; 4] {
  type AsUnsigned = [u32; 4];
  type AsSigned = [i32; 4];
}

impl ExprIntegralDataType for u32 {
  type AsUnsigned = u32;
  type AsSigned = i32;
}
impl ExprIntegralDataType for [u32; 2] {
  type AsUnsigned = [u32; 2];
  type AsSigned = [i32; 2];
}
impl ExprIntegralDataType for [u32; 3] {
  type AsUnsigned = [u32; 3];
  type AsSigned = [i32; 3];
}
impl ExprIntegralDataType for [u32; 4] {
  type AsUnsigned = [u32; 4];
  type AsSigned = [i32; 4];
}
