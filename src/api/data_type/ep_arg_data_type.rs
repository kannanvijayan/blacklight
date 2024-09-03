use super::{ArgDataType, ExprDataType, VarDataType};


/**
 * Trait that describes types which can serve as arguments to
 * shader entrypoints.
 */
pub trait EntryPointArgDataType
  : Sized + ArgDataType + VarDataType + ExprDataType
{}

impl EntryPointArgDataType for u32 {
}
impl EntryPointArgDataType for [u32; 2] {
}
impl EntryPointArgDataType for [u32; 3] {
}

