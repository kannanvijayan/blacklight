use crate::shader_repr::{
  body::ShaderBodyRepr,
  expression::ShaderExpressionRepr,
};

pub(crate) enum ShaderStatementRepr {
  Let(ShaderLetStmtRepr),
  Var(ShaderVarStmtRepr),
  If(ShaderIfStmtRepr),
  For(ShaderForStmtRepr),
  While(ShaderWhileStmtRepr),
  Break,
  Continue,
  Return(ShaderReturnStmtRepr),
  Expression(ShaderExpressionRepr),
}

pub(crate) struct ShaderLetStmtRepr {
  pub(crate) name: String,
  pub(crate) value: ShaderExpressionRepr,
}

pub(crate) struct ShaderVarStmtRepr {
  pub(crate) name: String,
  pub(crate) value: ShaderExpressionRepr,
}

pub(crate) struct ShaderIfStmtRepr {
  pub(crate) condition: ShaderExpressionRepr,
  pub(crate) if_body: Vec<ShaderBodyRepr>,
  pub(crate) else_body: Option<ShaderBodyRepr>,
}

pub(crate) struct ShaderForStmtRepr {
  pub(crate) init: Option<ShaderVarStmtRepr>,
  pub(crate) condition: Option<ShaderExpressionRepr>,
  pub(crate) step: Option<ShaderExpressionRepr>,
  pub(crate) body: ShaderBodyRepr,
}

pub(crate) struct ShaderWhileStmtRepr {
  pub(crate) condition: ShaderExpressionRepr,
  pub(crate) body: ShaderBodyRepr,
}

pub(crate) struct ShaderReturnStmtRepr {
  pub(crate) value: Option<ShaderExpressionRepr>,
}
