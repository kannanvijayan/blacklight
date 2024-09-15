use crate::api::data_type::ExprDataType;


/**
 * Data types that can be used in wgsl shader variable declarations.
 */
pub trait VarDataType: ExprDataType {
}

impl VarDataType for i32 {
}
impl VarDataType for [i32; 2] {
}
impl VarDataType for [i32; 3] {
}
impl VarDataType for [i32; 4] {
}

impl VarDataType for u32 {
}
impl VarDataType for [u32; 2] {
}
impl VarDataType for [u32; 3] {
}
impl VarDataType for [u32; 4] {
}

impl VarDataType for f32 {
}
impl VarDataType for [f32; 2] {
}
impl VarDataType for [f32; 3] {
}
impl VarDataType for [f32; 4] {
}
