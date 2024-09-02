use crate::api::data_type::ShLiteralDataValue;

/**
 * Represents an expression in a shader code block.
 */
#[derive(Clone, Debug)]
pub(crate) enum ExpressionModel {
  Literal(LiteralExprModel),
  Identifier(IdentifierExprModel),
  CmpOp(CmpOpExprModel),
}
impl ExpressionModel {
  /** Create a new identifier expression. */
  pub(crate) fn identifier<T: Into<String>>(name: T) -> Self {
    Self::Identifier(IdentifierExprModel::new(name.into()))
  }

  /** Create a new comparison operation expression. */
  pub(crate) fn cmp_op(
    lhs: ExpressionModel,
    rhs: ExpressionModel,
    op: CmpOp
  ) -> Self {
    Self::CmpOp(CmpOpExprModel::new(lhs, rhs, op))
  }
}
impl From<IdentifierExprModel> for ExpressionModel {
  fn from(expr: IdentifierExprModel) -> Self {
    Self::Identifier(expr)
  }
}

/**
 * Represents a literal expression.
 */
#[derive(Clone, Debug)]
pub(crate) struct LiteralExprModel(pub(crate) ShLiteralDataValue);
impl LiteralExprModel {
  /** Create a new literal expression. */
  pub(crate) fn new(value: ShLiteralDataValue) -> Self {
    LiteralExprModel(value)
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

/**
 * Represents a comparison operation expression.
 */
#[derive(Clone, Debug)]
pub(crate) struct CmpOpExprModel {
  // The left-hand side of the comparison.
  _lhs: Box<ExpressionModel>,

  // The right-hand side of the comparison.
  _rhs: Box<ExpressionModel>,

  // The comparison operator.
  _op: CmpOp,
}

#[derive(Clone, Debug)]
pub enum CmpOp { Eq, Ne, Lt, Le, Gt, Ge }
impl CmpOpExprModel {
  /** Create a new comparison operation expression. */
  pub(crate) fn new(lhs: ExpressionModel, rhs: ExpressionModel, op: CmpOp) -> Self {
    CmpOpExprModel {
      _lhs: Box::new(lhs),
      _rhs: Box::new(rhs),
      _op: op,
    }
  }
}
