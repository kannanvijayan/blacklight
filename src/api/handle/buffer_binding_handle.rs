use std::marker::PhantomData;
use crate::api::{
  data_type::BufferDataType,
  buffer_disposition::BufferDisposition,
};

/**
 * A handle to a buffer binding declared within a shader module.
 */
#[derive(Clone, Debug)]
pub struct BufferBindingHandle<'sh, DT, DISP>
  where DT: BufferDataType, DISP: BufferDisposition
{
  _name: String,
  _phantom: PhantomData<&'sh (DT, DISP)>,
}
impl<'sh, DT, DISP> BufferBindingHandle<'sh, DT, DISP>
  where DT: BufferDataType, DISP: BufferDisposition
{
  /** Create a new buffer binding handle. */
  pub(crate) fn new(name: String) -> Self {
    BufferBindingHandle { _name: name, _phantom: PhantomData }
  }
}
