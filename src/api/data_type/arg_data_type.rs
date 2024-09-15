use crate::api::data_type::{ ExprDataType, VarDataType };

/**
 * Data types that can be used to annotate wgsl entrypoint arguments.
 */
pub trait ArgDataType: VarDataType {
}
impl ArgDataType for i32 {
}
impl ArgDataType for [i32; 2] {
}
impl ArgDataType for [i32; 3] {
}
impl ArgDataType for [i32; 4] {
}

impl ArgDataType for u32 {
}
impl ArgDataType for [u32; 2] {
}
impl ArgDataType for [u32; 3] {
}
impl ArgDataType for [u32; 4] {
}

impl ArgDataType for f32 {
}
impl ArgDataType for [f32; 2] {
}
impl ArgDataType for [f32; 3] {
}
impl ArgDataType for [f32; 4] {
}

/**
 * Trait for tuples of shader argument data types.
 */
pub trait ShArgTupleDataType {
  /** The number of elements in the tuple. */
  const LENGTH: usize;
}
impl ShArgTupleDataType for () {
  const LENGTH: usize = 0;
}
impl<T> ShArgTupleDataType for (T,)
  where T: ArgDataType
{
  const LENGTH: usize = 1;
}
impl<T1, T2> ShArgTupleDataType for (T1, T2)
  where T1: ArgDataType, T2: ArgDataType
{
  const LENGTH: usize = 2;
}
impl<T1, T2, T3> ShArgTupleDataType for (T1, T2, T3)
  where T1: ArgDataType, T2: ArgDataType, T3: ArgDataType
{
  const LENGTH: usize = 3;
}
impl<T1, T2, T3, T4> ShArgTupleDataType for (T1, T2, T3, T4)
  where T1: ArgDataType, T2: ArgDataType, T3: ArgDataType, T4: ArgDataType
{
  const LENGTH: usize = 4;
}
