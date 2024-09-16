use std::collections::HashSet;
use crate::{
  api::data_type::{ DataTypeRepr, StructDataTypeRepr },
  model::IdentifierModel,
};

/**
 * A helper structure for collecting data types.
 */
pub(crate) struct DataTypeCollector {
  data_types: Vec<StructDataTypeRepr>,
  seen_names: HashSet<IdentifierModel>,
}
impl DataTypeCollector {
  /** Create a new data type collector. */
  pub(crate) fn new() -> Self {
    DataTypeCollector {
      data_types: Vec::new(),
      seen_names: HashSet::new(),
    }
  }

  /** Add a data type to the collector. */
  pub(crate) fn add_data_type(&mut self, data_type: DataTypeRepr) {
    if let Some(struct_data_type) = data_type.take_struct() {
      self.add_struct_data_type(struct_data_type);
    }
  }

  /** Get the collected data types. */
  pub(crate) fn take_data_types(self) -> Vec<StructDataTypeRepr> {
    self.data_types
  }

  fn add_struct_data_type(&mut self, struct_data_type: StructDataTypeRepr) {
    if self.seen_names.insert(struct_data_type.name().clone()) {
      for field in struct_data_type.fields() {
        self.add_data_type(field.data_type().clone());
      }
      self.data_types.push(struct_data_type);
    }
  }
}
