use std::marker::PhantomData;
use crate::api::data_type::ShBufferDataType;

/**
 * A handle to a buffer binding declared within a shader module.
 */
#[derive(Clone, Debug)]
pub struct BufferBindingHandle<'sh, DT: ShBufferDataType> {
  _name: String,
  _phantom: PhantomData<&'sh DT>,
}
impl<'sh, DT: ShBufferDataType> BufferBindingHandle<'sh, DT> {
  /** Create a new buffer binding handle. */
  pub(crate) fn new(name: String) -> Self {
    BufferBindingHandle { _name: name, _phantom: PhantomData }
  }
}
