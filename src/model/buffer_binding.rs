use crate::{
  api::{
    buffer_attributes::{ BufferDispositionRepr, BufferMemorySpaceRepr },
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

  // The memory space.
  memory_space: BufferMemorySpaceRepr,

  // The disposition of the buffer binding.
  disposition: BufferDispositionRepr,

  // The group of the buffer binding.
  group: u32,

  // The index of the buffer binding.
  index: u32,

  // The data type of the buffer binding.
  data_type: DataTypeRepr,

  // Whether the buffer type is a singleton, not array.
  is_singleton: bool,
}
impl BufferBindingModel {
  /** Create a new buffer binding. */
  pub(crate) fn new(
    name: IdentifierModel,
    memory_space: BufferMemorySpaceRepr,
    disposition: BufferDispositionRepr,
    group: u32,
    index: u32,
    data_type: DataTypeRepr,
    is_singleton: bool,
  ) -> BufferBindingModel {
    BufferBindingModel {
      name,
      memory_space,
      group,
      index,
      data_type,
      disposition,
      is_singleton,
    }
  }

  /** Get the name of the buffer binding. */
  pub(crate) fn name(&self) -> &IdentifierModel {
    &self.name
  }

  /** Get the memory space of the buffer binding. */
  pub(crate) fn memory_space(&self) -> BufferMemorySpaceRepr {
    self.memory_space
  }

  /** Get the disposition of the buffer binding. */
  pub(crate) fn disposition(&self) -> BufferDispositionRepr {
    self.disposition
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
  
  /** Check if the buffer contains a singleton entry. */
  pub(crate) fn is_singleton(&self) -> bool {
    self.is_singleton
  }

  /** Collect struct data types reference by this buffer into a vector. */
  pub(crate) fn collect_struct_data_types_into(&self,
    collector: &mut DataTypeCollector,
  ) {
    collector.add_data_type(self.data_type.clone());
  }
}
