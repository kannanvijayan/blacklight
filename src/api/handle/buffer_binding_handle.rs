use std::marker::PhantomData;
use crate::api::data_type::BufferDataType;

/**
 * A handle to a buffer binding declared within a shader module.
 */
#[derive(Clone, Debug)]
pub struct BufferBindingHandle<'sh, DT: BufferDataType> {
  _name: String,
  _phantom: PhantomData<&'sh DT>,
}
impl<'sh, DT: BufferDataType> BufferBindingHandle<'sh, DT> {
  /** Create a new buffer binding handle. */
  pub(crate) fn new(name: String) -> Self {
    BufferBindingHandle { _name: name, _phantom: PhantomData }
  }
}
