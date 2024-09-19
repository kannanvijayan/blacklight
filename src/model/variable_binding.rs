use crate::{
  api::data_type::DataTypeRepr,
  model::{ ExpressionModel, DataTypeCollector, IdentifierModel },
};

/**
 * A variable binding represents a `const`, `let`, or `var` declaration within
 * a shader module - either at the top level or within a function.
 */
#[derive(Clone, Debug)]
pub(crate) struct VariableBindingModel {
  // The name of the variable binding.
  name: IdentifierModel,

  // The variable disposition.
  disposition: VariableBindingDisposition,

  // The data type of the variable binding.
  data_type: DataTypeRepr,

  // The initial value of the variable binding.
  initial_value: Option<Box<ExpressionModel>>,
}
impl VariableBindingModel {
  /** Create a new variable binding model. */
  pub(crate) fn new(
    name: IdentifierModel,
    disposition: VariableBindingDisposition,
    data_type: DataTypeRepr,
    initial_value: Option<Box<ExpressionModel>>,
  ) -> Self {
    VariableBindingModel {
      name,
      disposition,
      data_type,
      initial_value,
    }
  }

  /** Get the name of the variable binding. */
  pub(crate) fn name(&self) -> &IdentifierModel {
    &self.name
  }

  /** Get the disposition of the variable binding. */
  pub(crate) fn disposition(&self) -> VariableBindingDisposition {
    self.disposition
  }

  /** Get the data type of the variable binding. */
  pub(crate) fn data_type(&self) -> &DataTypeRepr {
    &self.data_type
  }

  /** Get the initial value of the variable binding. */
  pub(crate) fn initial_value(&self) -> &Option<Box<ExpressionModel>> {
    &self.initial_value
  }

  pub(crate) fn collect_struct_data_types_into(&self,
    collector: &mut DataTypeCollector
  ) {
    if let Some(initial_value) = &self.initial_value {
      initial_value.collect_struct_data_types_into(collector);
    }
  }
}

/**
 * The disposition of a variable binding.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VariableBindingDisposition {
  // A constant value.
  Const,

  // A mutable value.
  Var,

  // An immutable value.
  Let,
}
impl VariableBindingDisposition {
  /** Get the string representation of the variable binding disposition. */
  pub(crate) fn wgsl_source(&self) -> &'static str {
    match self {
      VariableBindingDisposition::Const => "const",
      VariableBindingDisposition::Var => "var",
      VariableBindingDisposition::Let => "let",
    }
  }
}
