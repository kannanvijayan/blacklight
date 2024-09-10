
/**
 * Data types that can be the result of expressions.
 */
pub trait ExprDataType: 'static + Sized {
}

impl ExprDataType for bool {
}

impl ExprDataType for i32 {
}
impl ExprDataType for [i32; 2] {
}
impl ExprDataType for [i32; 3] {
}
impl ExprDataType for [i32; 4] {
}

impl ExprDataType for u32 {
}
impl ExprDataType for [u32; 2] {
}
impl ExprDataType for [u32; 3] {
}
impl ExprDataType for [u32; 4] {
}

impl ExprDataType for f32 {
}
impl ExprDataType for [f32; 2] {
}
impl ExprDataType for [f32; 3] {
}
impl ExprDataType for [f32; 4] {
}


/**
 * Expr data types that are numeric in nature (i.e. are field data types).
 */
pub trait ExprNumericDataType: ExprDataType {
}

impl ExprNumericDataType for i32 {
}
impl ExprNumericDataType for [i32; 2] {
}
impl ExprNumericDataType for [i32; 3] {
}
impl ExprNumericDataType for [i32; 4] {
}

impl ExprNumericDataType for u32 {
}
impl ExprNumericDataType for [u32; 2] {
}
impl ExprNumericDataType for [u32; 3] {
}
impl ExprNumericDataType for [u32; 4] {
}

impl ExprNumericDataType for f32 {
}
impl ExprNumericDataType for [f32; 2] {
}
impl ExprNumericDataType for [f32; 3] {
}
impl ExprNumericDataType for [f32; 4] {
}
