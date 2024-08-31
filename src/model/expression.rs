/**
 * Represents an expression in a shader code block.
 */
#[derive(Clone, Debug)]
pub(crate) enum ExpressionModel {
  Identifier(IdentifierExprModel),
}
impl ExpressionModel {
  /** Create a new identifier expression. */
  pub(crate) fn identifier<T: Into<String>>(name: T) -> Self {
    Self::Identifier(IdentifierExprModel::new(name.into()))
  }
}
impl From<IdentifierExprModel> for ExpressionModel {
  fn from(expr: IdentifierExprModel) -> Self {
    Self::Identifier(expr)
  }
}

/**
 * Represents an identifier expression.
 */
#[derive(Clone, Debug)]
pub(crate) struct IdentifierExprModel {
  // The name of the identifier being referenced.
  _name: String,

  // TODO: add type model reference.
}
impl IdentifierExprModel {
  /** Create a new identifier expression. */
  pub(crate) fn new(name: String) -> Self {
    IdentifierExprModel { _name: name }
  }
}
