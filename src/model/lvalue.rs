use crate::{
  api::data_type::DataTypeRepr,
  model::{
    BufferReadExprModel,
    ExpressionModel,
    IdentifierExprModel,
    IdentifierModel,
  },
};

/**
 * A model of an expression that is can be used as an lvalue.
 */
#[derive(Clone, Debug)]
pub(crate) enum LvalueModel {
  // A variable reference.
  Variable(String, DataTypeRepr),

  // A buffer element reference.
  BufferElement(String, Box<ExpressionModel>, DataTypeRepr),
}
impl LvalueModel {
  /** Create a new lvalue variable model. */
  pub(crate) fn new_variable(name: String, data_type: DataTypeRepr) -> Self {
    LvalueModel::Variable(name, data_type)
  }

  /** Create a new lvalue model for a buffer element reference. */
  pub(crate) fn new_buffer_element(
    buffer_name: String,
    index: ExpressionModel,
    data_type: DataTypeRepr,
  ) -> Self {
    LvalueModel::BufferElement(buffer_name, Box::new(index), data_type)
  }

  /** Create a read expression model for this lvalue. */
  pub(crate) fn read_expr(&self) -> ExpressionModel {
    match self {
      LvalueModel::Variable(name, data_type) => {
        let identifier_model  = IdentifierModel::new(&name);
        let identifier_expr_model = IdentifierExprModel::new(
          identifier_model,
          data_type.clone(),
        );
        ExpressionModel::Identifier(identifier_expr_model)
      },
      LvalueModel::BufferElement(buffer_name, index, data_type) => {
        let buffer_read_expr_model =
          BufferReadExprModel::new(
            buffer_name.clone(),
            index.clone(),
            data_type.clone()
          );
        ExpressionModel::BufferRead(buffer_read_expr_model)
      }
    }
  }
}
