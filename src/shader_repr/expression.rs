use crate::shader_repr::native_value::ShaderNativeValueRepr;

pub(crate) enum ShaderExpressionRepr {
  BinaryOp(ShaderBinaryOpExprRepr),
  UnaryOp(ShaderUnaryOpExprRepr),
  FunctionCall(ShaderFunctionCallExprRepr),
  MemberAccess(ShaderMemberAccessExprRepr),
  ArrayAccess(ShaderArrayAccessExprRepr),
  Constant(ShaderConstantExprRepr),
  Variable(ShaderVariableExprRepr),
}

pub(crate) struct ShaderBinaryOpExprRepr {
  pub(crate) op: ShaderBinaryOp,
  pub(crate) left: Box<ShaderExpressionRepr>,
  pub(crate) right: Box<ShaderExpressionRepr>,
}

pub(crate) enum ShaderBinaryOp {
  Add, Sub, Mul, Div, Mod,
  And, Or, Xor, Shl, Shr,
  Eq, Ne, Lt, Le, Gt, Ge,
}

pub(crate) struct ShaderUnaryOpExprRepr {
  pub(crate) op: ShaderUnaryOp,
  pub(crate) operand: Box<ShaderExpressionRepr>,
}

pub(crate) enum ShaderUnaryOp { Neg, Not }

pub(crate) struct ShaderFunctionCallExprRepr {
  pub(crate) callee: Box<ShaderExpressionRepr>,
  pub(crate) args: Vec<ShaderExpressionRepr>,
}

pub(crate) struct ShaderMemberAccessExprRepr {
  pub(crate) base: Box<ShaderExpressionRepr>,
  pub(crate) member: String,
}

pub(crate) struct ShaderArrayAccessExprRepr {
  pub(crate) base: Box<ShaderExpressionRepr>,
  pub(crate) index: Box<ShaderExpressionRepr>,
}

pub(crate) struct ShaderConstantExprRepr {
  pub(crate) value: ShaderNativeValueRepr,
}

pub(crate) struct ShaderVariableExprRepr {
  pub(crate) name: String,
}
