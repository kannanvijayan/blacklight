use crate::{
  api::data_type::DataTypeRepr,
  model::{ CodeBlockModel, DataTypeCollector, IdentifierModel },
};

/**
 * Models an entry point in a shader module.
 */
#[derive(Clone, Debug)]
pub(crate) struct FunctionModel {
  // The name of the entry point.
  name: IdentifierModel,

  // The argument names.
  arg_names: Vec<IdentifierModel>,

  // The argument data types.
  arg_data_types: Vec<DataTypeRepr>,

  // The return data type.
  return_data_type: Option<DataTypeRepr>,

  // The code block for the entry point.
  code_block: CodeBlockModel,
}
impl FunctionModel {
  /** Create a new entry point. */
  pub(crate) fn new(
    name: IdentifierModel,
    arg_names: Vec<IdentifierModel>,
    arg_data_types: Vec<DataTypeRepr>,
    return_data_type: Option<DataTypeRepr>,
    code_block: CodeBlockModel,
  ) -> FunctionModel {
    FunctionModel {
      name,
      arg_names,
      arg_data_types,
      return_data_type,
      code_block,
    }
  }

  /** Get the name of the entry point. */
  pub(crate) fn name(&self) -> &IdentifierModel {
    &self.name
  }

  /** Get the argument names. */
  pub(crate) fn arg_names(&self) -> &[IdentifierModel] {
    &self.arg_names
  }

  /** Get the argument data types. */
  pub(crate) fn arg_data_types(&self) -> &[DataTypeRepr] {
    &self.arg_data_types
  }

  /** Get the return data type. */
  pub(crate) fn return_data_type(&self) -> Option<&DataTypeRepr> {
    self.return_data_type.as_ref()
  }

  /** Get the code block for the entry point. */
  pub(crate) fn code_block(&self) -> &CodeBlockModel {
    &self.code_block
  }

  /** Collect struct data types reference by this buffer into a vector. */
  pub(crate) fn collect_struct_data_types_into(&self,
    collector: &mut DataTypeCollector,
  ) {
    for arg_data_type in &self.arg_data_types {
      collector.add_data_type(arg_data_type.clone());
    }
    if let Some(return_data_type) = &self.return_data_type {
      collector.add_data_type(return_data_type.clone());
    }
    self.code_block.collect_struct_data_types_into(collector);
  }
}
