use super::{ShArgDataType, ShExprDataType, ShVarDataType};


/**
 * Trait that describes types which can serve as arguments to
 * shader entrypoints.
 */
pub trait ShEpArgDataType
  : Sized + ShArgDataType + ShVarDataType + ShExprDataType
{}

impl ShEpArgDataType for u32 {
}
impl ShEpArgDataType for [u32; 2] {
}
impl ShEpArgDataType for [u32; 3] {
}

