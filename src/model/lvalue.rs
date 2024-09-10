use crate::model::{ BufferReadExprModel, ExpressionModel, IdentifierExprModel };

/**
 * A model of an expression that is can be used as an lvalue.
 */
#[derive(Clone, Debug)]
pub(crate) enum LvalueModel {
  // A variable reference.
  Variable(String),

  // A buffer element reference.
  BufferElement(String, Box<ExpressionModel>),
}
impl LvalueModel {
  /** Create a new lvalue variable model. */
  pub(crate) fn new_variable(name: String) -> Self {
    LvalueModel::Variable(name)
  }

  /** Create a new lvalue model for a buffer element reference. */
  pub(crate) fn new_buffer_element(
    buffer_name: String,
    index: ExpressionModel
  ) -> Self {
    LvalueModel::BufferElement(buffer_name, Box::new(index))
  }

  /** Create a read expression model for this lvalue. */
  pub(crate) fn read_expr(&self) -> ExpressionModel {
    match self {
      LvalueModel::Variable(name) => {
        let identifier_expr_model = IdentifierExprModel::new(name.clone());
        ExpressionModel::Identifier(identifier_expr_model)
      },
      LvalueModel::BufferElement(buffer_name, index) => {
        let buffer_read_expr_model =
          BufferReadExprModel::new(buffer_name.clone(), index.clone());
        ExpressionModel::BufferRead(buffer_read_expr_model)
      }
    }
  }
}
