
/**
 * Trait characterizing types that can serve as the "return" type
 * of a shader procedure.  This includes shader functions and entry points.
 */
pub trait ProcResultType {
}

impl ProcResultType for () {
}

impl ProcResultType for bool {
}

impl ProcResultType for i32 {
}
impl ProcResultType for [i32; 2] {
}
impl ProcResultType for [i32; 3] {
}
impl ProcResultType for [i32; 4] {
}

impl ProcResultType for u32 {
}
impl ProcResultType for [u32; 2] {
}
impl ProcResultType for [u32; 3] {
}
impl ProcResultType for [u32; 4] {
}

impl ProcResultType for f32 {
}
impl ProcResultType for [f32; 2] {
}
impl ProcResultType for [f32; 3] {
}
impl ProcResultType for [f32; 4] {
}
