use super::ShExprDataType;


/**
 * Data types that can be used in wgsl shader variable declarations.
 */
pub trait ShVarDataType: Sized + ShExprDataType {
}

impl ShVarDataType for i32 {
}
impl ShVarDataType for [i32; 2] {
}
impl ShVarDataType for [i32; 3] {
}
impl ShVarDataType for [i32; 4] {
}

impl ShVarDataType for u32 {
}
impl ShVarDataType for [u32; 2] {
}
impl ShVarDataType for [u32; 3] {
}
impl ShVarDataType for [u32; 4] {
}

impl ShVarDataType for f32 {
}
impl ShVarDataType for [f32; 2] {
}
impl ShVarDataType for [f32; 3] {
}
impl ShVarDataType for [f32; 4] {
}
