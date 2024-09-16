use crate::api::data_type::{ DataTypeRepr, Struct, StructMappedDataType };

/**
 * Trait characterizing types that can serve as the "return" type
 * of a shader procedure.  This includes shader functions and entry points.
 */
pub trait ProcResultType: 'static + Sized {
  /** The runtime representation of the type, if available.. */
  fn proc_result_repr() -> Option<DataTypeRepr>;
}

impl ProcResultType for () {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    None
  }
}

impl ProcResultType for bool {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_bool())
  }
}

impl ProcResultType for i32 {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_i32())
  }
}
impl ProcResultType for [i32; 2] {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_vec2_i32())
  }
}
impl ProcResultType for [i32; 3] {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_vec3_i32())
  }
}
impl ProcResultType for [i32; 4] {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_vec4_i32())
  }
}

impl ProcResultType for u32 {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_u32())
  }
}
impl ProcResultType for [u32; 2] {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_vec2_u32())
  }
}
impl ProcResultType for [u32; 3] {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_vec3_u32())
  }
}
impl ProcResultType for [u32; 4] {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_vec4_u32())
  }
}

impl ProcResultType for f32 {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_f32())
  }
}
impl ProcResultType for [f32; 2] {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_vec2_f32())
  }
}
impl ProcResultType for [f32; 3] {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_vec3_f32())
  }
}
impl ProcResultType for [f32; 4] {
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(DataTypeRepr::new_vec4_f32())
  }
}

impl<T> ProcResultType for Struct<T>
  where T: Copy + StructMappedDataType
{
  fn proc_result_repr() -> Option<DataTypeRepr> {
    Some(Struct::<T>::make_repr())
  }
}
