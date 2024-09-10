use crate::api::data_type::LiteralDataValue;

/**
 * Represents an expression in a shader code block.
 */
#[derive(Clone, Debug)]
pub(crate) enum ExpressionModel {
  Literal(LiteralExprModel),
  Identifier(IdentifierExprModel),
  CmpOp(CmpOpExprModel),
  BinOp(BinOpExprModel),
  BufferRead(BufferReadExprModel),
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

/**
 * Represents a read from a buffer.
 */
#[derive(Clone, Debug)]
pub(crate) struct BufferReadExprModel {
  // The buffer binding being read from.
  buffer_name: String,

  // The index of the element being read.
  index: Box<ExpressionModel>,
}
impl BufferReadExprModel {
  /** Create a new buffer read expression. */
  pub(crate) fn new(buffer_name: String, index: Box<ExpressionModel>) -> Self {
    BufferReadExprModel { buffer_name, index }
  }

  /** Get the name of the buffer binding being read from. */
  pub(crate) fn buffer_name(&self) -> &str {
    &self.buffer_name
  }

  /** Get the index of the element being read. */
  pub(crate) fn index(&self) -> &ExpressionModel {
    &self.index
  }
}

/**
 * Represents a binary operation expression.
 */
#[derive(Clone, Debug)]
pub(crate) struct BinOpExprModel {
  // The left-hand side of the binary operation.
  lhs: Box<ExpressionModel>,

  // The right-hand side of the binary operation.
  rhs: Box<ExpressionModel>,

  // The binary operator.
  op: BinOp,
}

impl BinOpExprModel {
  /** Create a new binary operation expression. */
  pub(crate) fn new(lhs: ExpressionModel, rhs: ExpressionModel, op: BinOp) -> Self {
    BinOpExprModel {
      lhs: Box::new(lhs),
      rhs: Box::new(rhs),
      op,
    }
  }

  /** Get the left-hand side of the binary operation. */
  pub(crate) fn lhs(&self) -> &ExpressionModel {
    &self.lhs
  }

  /** Get the right-hand side of the binary operation. */
  pub(crate) fn rhs(&self) -> &ExpressionModel {
    &self.rhs
  }

  /** Get the binary operator. */
  pub(crate) fn op(&self) -> BinOp {
    self.op
  }
}

#[derive(Clone, Copy, Debug)]
pub enum BinOp { Add, Sub, Mul, Div, Rem }
impl BinOp {
  /** Get the string representation of the binary operator */
  pub(crate) fn operator_str(self) -> &'static str {
    match self {
      BinOp::Add => "+",
      BinOp::Sub => "-",
      BinOp::Mul => "*",
      BinOp::Div => "/",
      BinOp::Rem => "%",
    }
  }
}
