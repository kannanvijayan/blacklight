use crate::model::ExpressionModel;

/**
 * Represents a statement in a shader code block.
 */
#[derive(Clone, Debug)]
pub(crate) enum StatementModel {
  VarDecl(VarDeclStmtModel),
  Assign(AssignStmtModel),
  Expr(ExprStmtModel),
  Return(ReturnStmtModel),
}

/**
 * Represents a variable declaration statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct VarDeclStmtModel {
  // The name of the variable being declared.
  _name: String,

  // The expression being assigned to the variable.
  _expression: ExpressionModel,
}
impl VarDeclStmtModel {
  /** Create a new variable declaration statement. */
  pub(crate) fn new(name: String, expression: ExpressionModel) -> Self {
    VarDeclStmtModel { _name: name, _expression: expression }
  }
}

/**
 * Represents an assignment statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct AssignStmtModel {
  // The name of the variable being assigned to.
  _name: String,

  // The expression being assigned to the variable.
  _expression: ExpressionModel,
}
impl AssignStmtModel {
  /** Create a new assignment statement. */
  pub(crate) fn new(name: String, expression: ExpressionModel) -> Self {
    AssignStmtModel { _name: name, _expression: expression }
  }
}

/**
 * Represents an expression statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct ExprStmtModel {
  _expression: ExpressionModel,
}
impl ExprStmtModel {
  /** Create a new expression statement. */
  pub(crate) fn new(expression: ExpressionModel) -> Self {
    ExprStmtModel { _expression: expression }
  }
}

/**
 * Represents a return statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct ReturnStmtModel {
  // The expression being returned.
  _expression: Option<ExpressionModel>,
}
impl ReturnStmtModel {
  /** Create a new return statement. */
  pub(crate) fn new(expression: Option<ExpressionModel>) -> Self {
    ReturnStmtModel { _expression: expression }
  }
}
