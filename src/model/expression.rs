use crate::api::data_type::LiteralDataValue;

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
pub(crate) struct LiteralExprModel {
  value: LiteralDataValue,
}
impl LiteralExprModel {
  /** Create a new literal expression. */
  pub(crate) fn new(value: LiteralDataValue) -> Self {
    LiteralExprModel { value }
  }

  /** Get the value of the literal. */
  pub(crate) fn value(&self) -> &LiteralDataValue {
    &self.value
  }
}

/**
 * Represents an identifier expression.
 */
#[derive(Clone, Debug)]
pub(crate) struct IdentifierExprModel {
  // The name of the identifier being referenced.
  name: String,
}
impl IdentifierExprModel {
  /** Create a new identifier expression. */
  pub(crate) fn new(name: String) -> Self {
    IdentifierExprModel { name }
  }

  /** Get the name of the identifier. */
  pub(crate) fn name(&self) -> &str {
    &self.name
  }
}

/**
 * Represents a comparison operation expression.
 */
#[derive(Clone, Debug)]
pub(crate) struct CmpOpExprModel {
  // The left-hand side of the comparison.
  lhs: Box<ExpressionModel>,

  // The right-hand side of the comparison.
  rhs: Box<ExpressionModel>,

  // The comparison operator.
  op: CmpOp,
}

impl CmpOpExprModel {
  /** Create a new comparison operation expression. */
  pub(crate) fn new(lhs: ExpressionModel, rhs: ExpressionModel, op: CmpOp) -> Self {
    CmpOpExprModel {
      lhs: Box::new(lhs),
      rhs: Box::new(rhs),
      op,
    }
  }

  /** Get the left-hand side of the comparison. */
  pub(crate) fn lhs(&self) -> &ExpressionModel {
    &self.lhs
  }

  /** Get the right-hand side of the comparison. */
  pub(crate) fn rhs(&self) -> &ExpressionModel {
    &self.rhs
  }

  /** Get the comparison operator. */
  pub(crate) fn op(&self) -> CmpOp {
    self.op
  }
}

#[derive(Clone, Copy, Debug)]
pub enum CmpOp { Eq, Ne, Lt, Le, Gt, Ge }
impl CmpOp {
  /** Get the string representation of the comparison operator */
  pub(crate) fn operator_str(self) -> &'static str {
    match self {
      CmpOp::Eq => "==",
      CmpOp::Ne => "!=",
      CmpOp::Lt => "<",
      CmpOp::Le => "<=",
      CmpOp::Gt => ">",
      CmpOp::Ge => ">=",
    }
  }
}
