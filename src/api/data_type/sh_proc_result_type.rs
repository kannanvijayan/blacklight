
/**
 * Trait characterizing types that can serve as the "return" type
 * of a shader procedure.  This includes shader functions and entry points.
 */
pub trait ShProcResultType {
}

impl ShProcResultType for () {
}

impl ShProcResultType for bool {
}

impl ShProcResultType for i32 {
}
impl ShProcResultType for [i32; 2] {
}
impl ShProcResultType for [i32; 3] {
}
impl ShProcResultType for [i32; 4] {
}

impl ShProcResultType for u32 {
}
impl ShProcResultType for [u32; 2] {
}
impl ShProcResultType for [u32; 3] {
}
impl ShProcResultType for [u32; 4] {
}

impl ShProcResultType for f32 {
}
impl ShProcResultType for [f32; 2] {
}
impl ShProcResultType for [f32; 3] {
}
impl ShProcResultType for [f32; 4] {
}
