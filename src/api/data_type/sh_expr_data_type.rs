
/**
 * Data types that can be the result of expressions.
 */
pub trait ShExprDataType: 'static + Sized {
}

impl ShExprDataType for bool {
}

impl ShExprDataType for i32 {
}
impl ShExprDataType for [i32; 2] {
}
impl ShExprDataType for [i32; 3] {
}
impl ShExprDataType for [i32; 4] {
}

impl ShExprDataType for u32 {
}
impl ShExprDataType for [u32; 2] {
}
impl ShExprDataType for [u32; 3] {
}
impl ShExprDataType for [u32; 4] {
}

impl ShExprDataType for f32 {
}
impl ShExprDataType for [f32; 2] {
}
impl ShExprDataType for [f32; 3] {
}
impl ShExprDataType for [f32; 4] {
}
