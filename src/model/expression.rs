use crate::{
  api::data_type::{ DataTypeRepr, LiteralDataValue },
  model::{ DataTypeCollector, IdentifierModel },
};

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
  StructFieldRead(StructFieldReadModel),
  FunctionCall(FunctionCallExprModel),
}
impl ExpressionModel {
  pub(crate) fn data_type(&self) -> DataTypeRepr {
    match self {
      ExpressionModel::Literal(literal_expr) => {
        literal_expr.value().data_type_repr()
      },
      ExpressionModel::Identifier(identifier_expr) => {
        identifier_expr.data_type().clone()
      },
      ExpressionModel::CmpOp(_) => DataTypeRepr::new_bool(),
      ExpressionModel::BinOp(bin_op_expr) => {
        bin_op_expr.lhs().data_type().clone()
      },
      ExpressionModel::BufferRead(buffer_read_expr) => {
        buffer_read_expr.data_type().clone()
      },
      ExpressionModel::StructFieldRead(struct_field_read) => {
        struct_field_read.data_type().clone()
      },
      ExpressionModel::FunctionCall(function_call) => {
        function_call.return_data_type().clone()
      },
    }
  }

  /** Collect struct data types reference by this buffer into a vector. */
  pub(crate) fn collect_struct_data_types_into(&self,
    collector: &mut DataTypeCollector,
  ) {
    match self {
      ExpressionModel::Literal(literal_expr) => {
        collector.add_data_type(literal_expr.value().data_type_repr());
      },
      ExpressionModel::Identifier(ident_expr) => {
        collector.add_data_type(ident_expr.data_type().clone());
      },
      ExpressionModel::CmpOp(cmp_op_expr) => {
        cmp_op_expr.lhs().collect_struct_data_types_into(collector);
        cmp_op_expr.rhs().collect_struct_data_types_into(collector);
      },
      ExpressionModel::BinOp(bin_op_expr) => {
        bin_op_expr.lhs().collect_struct_data_types_into(collector);
        bin_op_expr.rhs().collect_struct_data_types_into(collector);
      },
      ExpressionModel::BufferRead(buffer_read_expr) => {
        collector.add_data_type(buffer_read_expr.data_type().clone());
      },
      ExpressionModel::StructFieldRead(struct_field_read) => {
        collector.add_data_type(struct_field_read.data_type().clone());
      },
      ExpressionModel::FunctionCall(function_call) => {
        collector.add_data_type(function_call.return_data_type().clone());
        for arg in function_call.arguments() {
          arg.as_ref().collect_struct_data_types_into(collector);
        }
      },
    }
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
  identifier: IdentifierModel,

  // The data type of the identifier.
  data_type: DataTypeRepr,
}
impl IdentifierExprModel {
  /** Create a new identifier expression. */
  pub(crate) fn new(
    identifier: IdentifierModel,
    data_type: DataTypeRepr,
  ) -> Self {
    IdentifierExprModel { identifier, data_type }
  }

  /** Get the name of the identifier. */
  pub(crate) fn identifier(&self) -> &IdentifierModel {
    &self.identifier
  }

  /** Get the data type of the identifier. */
  pub(crate) fn data_type(&self) -> &DataTypeRepr {
    &self.data_type
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
  pub(crate) fn new(
    lhs: Box<ExpressionModel>,
    rhs: Box<ExpressionModel>,
    op: CmpOp
  ) -> Self {
    CmpOpExprModel { lhs, rhs, op }
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
  buffer_name: IdentifierModel,

  // The index of the element being read.
  index: Box<ExpressionModel>,

  // The element data type of the buffer binding.
  data_type: DataTypeRepr,
}
impl BufferReadExprModel {
  /** Create a new buffer read expression. */
  pub(crate) fn new(
    buffer_name: IdentifierModel,
    index: Box<ExpressionModel>,
    data_type: DataTypeRepr,
  ) -> Self {
    BufferReadExprModel { buffer_name, index, data_type }
  }

  /** Get the name of the buffer binding being read from. */
  pub(crate) fn buffer_name(&self) -> &IdentifierModel {
    &self.buffer_name
  }

  /** Get the index of the element being read. */
  pub(crate) fn index(&self) -> &ExpressionModel {
    &self.index
  }

  /** Get the element data type of the buffer binding. */
  pub(crate) fn data_type(&self) -> &DataTypeRepr {
    &self.data_type
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
  pub(crate) fn new(
    lhs: Box<ExpressionModel>,
    rhs: Box<ExpressionModel>,
    op: BinOp,
  ) -> Self {
    BinOpExprModel { lhs, rhs, op }
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
pub enum BinOp {
  Add, Sub, Mul, Div, Rem,
  BitOr, BitAnd, BitXor, Shl, Shr,
}
impl BinOp {
  /** Get the string representation of the binary operator */
  pub(crate) fn operator_str(self) -> &'static str {
    match self {
      BinOp::Add => "+",
      BinOp::Sub => "-",
      BinOp::Mul => "*",
      BinOp::Div => "/",
      BinOp::Rem => "%",
      BinOp::BitOr => "|",
      BinOp::BitAnd => "&",
      BinOp::BitXor => "^",
      BinOp::Shl => "<<",
      BinOp::Shr => ">>",
    }
  }
}

/**
 * Represents a read from a field of a struct.
 */
#[derive(Clone, Debug)]
pub(crate) struct StructFieldReadModel {
  // The struct expression being read from.
  struct_expr: Box<ExpressionModel>,

  // The name of the field being read.
  field_name: IdentifierModel,

  // The data type of the field being read.
  data_type: DataTypeRepr,
}
impl StructFieldReadModel {
  /** Create a new struct field read expression. */
  pub(crate) fn new(
    struct_expr: Box<ExpressionModel>,
    field_name: IdentifierModel,
    data_type: DataTypeRepr,
  ) -> Self {
    StructFieldReadModel { struct_expr, field_name, data_type }
  }

  /** Get the struct expression being read from. */
  pub(crate) fn struct_expr(&self) -> &ExpressionModel {
    &self.struct_expr
  }

  /** Get the name of the field being read. */
  pub(crate) fn field_name(&self) -> &IdentifierModel {
    &self.field_name
  }

  /** Get the data type of the field being read. */
  pub(crate) fn data_type(&self) -> &DataTypeRepr {
    &self.data_type
  }
}

/**
 * Represents a function call expression.
 */
#[derive(Clone, Debug)]
pub(crate) struct FunctionCallExprModel {
  // The name of the function being called.
  function_name: IdentifierModel,

  // The arguments to the function.
  arguments: Vec<Box<ExpressionModel>>,

  // The return data type of the function.
  return_data_type: DataTypeRepr,
}
impl FunctionCallExprModel {
  /** Create a new function call expression. */
  pub(crate) fn new(
    function_name: IdentifierModel,
    arguments: Vec<Box<ExpressionModel>>,
    return_data_type: DataTypeRepr,
  ) -> Self {
    FunctionCallExprModel { function_name, arguments, return_data_type }
  }

  /** Get the name of the function being called. */
  pub(crate) fn function_name(&self) -> &IdentifierModel {
    &self.function_name
  }

  /** Get the arguments to the function. */
  pub(crate) fn arguments(&self) -> &[impl AsRef<ExpressionModel>] {
    &self.arguments
  }

  /** Get the return data type of the function. */
  pub(crate) fn return_data_type(&self) -> &DataTypeRepr {
    &self.return_data_type
  }
}
