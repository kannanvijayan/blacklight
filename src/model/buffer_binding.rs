use crate::{
  api::{
    buffer_disposition::BufferDispositionRepr,
    data_type::DataTypeRepr,
  },
  model::{ IdentifierModel, DataTypeCollector },
};

/**
 * A model of a binding to a buffer within a shader module.
 */
#[derive(Clone, Debug)]
pub struct BufferBindingModel {
  // The name of the buffer binding.
  name: IdentifierModel,

  // The group of the buffer binding.
  group: u32,

  // The index of the buffer binding.
  index: u32,

  // The data type of the buffer binding.
  data_type: DataTypeRepr,

  // The disposition of the buffer binding.
  disposition: BufferDispositionRepr,
}
impl BufferBindingModel {
  /** Create a new buffer binding. */
  pub(crate) fn new(
    name: IdentifierModel,
    group: u32,
    index: u32,
    data_type: DataTypeRepr,
    disposition: BufferDispositionRepr,
  ) -> BufferBindingModel {
    BufferBindingModel {
      name: name,
      group: group,
      index: index,
      data_type: data_type,
      disposition: disposition,
    }
  }

  /** Get the name of the buffer binding. */
  pub(crate) fn name(&self) -> &IdentifierModel {
    &self.name
  }

  /** Get the group of the buffer binding. */
  pub(crate) fn group(&self) -> u32 {
    self.group
  }

  /** Get the index of the buffer binding. */
  pub(crate) fn index(&self) -> u32 {
    self.index
  }

  /** Get the data type of the buffer binding. */
  pub(crate) fn data_type(&self) -> &DataTypeRepr {
    &self.data_type
  }

  /** Get the disposition of the buffer binding. */
  pub(crate) fn disposition(&self) -> BufferDispositionRepr {
    self.disposition
  }

  /** Collect struct data types reference by this buffer into a vector. */
  pub(crate) fn collect_struct_data_types_into(&self,
    collector: &mut DataTypeCollector,
  ) {
    collector.add_data_type(self.data_type.clone());
  }
}
