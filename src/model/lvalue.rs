use crate::{
  api::data_type::DataTypeRepr,
  model::{ ExpressionModel, IdentifierModel },
};

/**
 * A model of an expression that is can be used as an lvalue.
 */
#[derive(Clone, Debug)]
pub(crate) enum LvalueModel {
  // A variable reference.
  Variable(IdentifierModel, DataTypeRepr),

  // A buffer element reference.
  BufferElement(IdentifierModel, Box<ExpressionModel>, DataTypeRepr),

  // A struct field reference.
  StructField(Box<ExpressionModel>, IdentifierModel, DataTypeRepr),
}
impl LvalueModel {
  /** Create a new lvalue variable model. */
  pub(crate) fn new_variable(
    name: IdentifierModel,
    data_type: DataTypeRepr,
  ) -> Self {
    LvalueModel::Variable(name, data_type)
  }

  /** Create a new lvalue model for a buffer element reference. */
  pub(crate) fn new_buffer_element(
    buffer_name: IdentifierModel,
    index: Box<ExpressionModel>,
    data_type: DataTypeRepr,
  ) -> Self {
    LvalueModel::BufferElement(buffer_name, index, data_type)
  }

  /** Create a new lvalue model for a struct field. */
  pub(crate) fn new_struct_field(
    base: Box<ExpressionModel>,
    field_name: IdentifierModel,
    data_type: DataTypeRepr,
  ) -> Self {
    LvalueModel::StructField(base, field_name, data_type)
  }
}
