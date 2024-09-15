use crate::api::{
  block_dims::BlockDims,
  data_type::{ ArgDataType, ExprDataType, VarDataType },
};



/**
 * Trait that describes types which can serve as arguments to
 * shader entrypoints.
 */
pub trait EntryPointArgDataType : ArgDataType {
  fn to_block_dims(&self) -> BlockDims;
}

impl EntryPointArgDataType for u32 {
  fn to_block_dims(&self) -> BlockDims {
    BlockDims::OneD(*self)
  }
}
impl EntryPointArgDataType for [u32; 2] {
  fn to_block_dims(&self) -> BlockDims {
    BlockDims::TwoD(*self)
  }
}
impl EntryPointArgDataType for [u32; 3] {
  fn to_block_dims(&self) -> BlockDims {
    BlockDims::ThreeD(*self)
  }
}

