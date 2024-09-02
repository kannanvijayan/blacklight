use crate::api::data_type::ShBufferDataTypeRepr;

/**
 * A model of a binding to a buffer within a shader module.
 */
#[derive(Clone, Debug)]
pub struct BufferBindingModel {
  // The name of the buffer binding.
  _name: String,

  // The group of the buffer binding.
  _group: u32,

  // The index of the buffer binding.
  _index: u32,

  // The data type of the buffer binding.
  _data_type: ShBufferDataTypeRepr,
}
impl BufferBindingModel {
  /** Create a new buffer binding. */
  pub(crate) fn new(
    name: String,
    group: u32,
    index: u32,
    data_type: ShBufferDataTypeRepr,
  ) -> BufferBindingModel {
    BufferBindingModel {
      _name: name,
      _group: group,
      _index: index,
      _data_type: data_type,
    }
  }
}
