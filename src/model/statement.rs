use crate::model::{ CodeBlockModel, ExpressionModel };

/**
 * Represents a statement in a shader code block.
 */
#[derive(Clone, Debug)]
pub(crate) enum StatementModel {
  VarDecl(VarDeclStmtModel),
  Assign(AssignStmtModel),
  IfElse(IfElseStmtModel),
  Expr(ExprStmtModel),
  Return(ReturnStmtModel),
}

/**
 * Represents a variable declaration statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct VarDeclStmtModel {
  // The name of the variable being declared.
  name: String,

  // The expression being assigned to the variable.
  expression: ExpressionModel,
}
impl VarDeclStmtModel {
  /** Create a new variable declaration statement. */
  pub(crate) fn new(name: String, expression: ExpressionModel) -> Self {
    VarDeclStmtModel { name, expression }
  }

  /** Get the name of the variable being declared. */
  pub(crate) fn name(&self) -> &str {
    &self.name
  }

  /** Get the expression being assigned to the variable. */
  pub(crate) fn expression(&self) -> &ExpressionModel {
    &self.expression
  }
}

/**
 * Represents an assignment statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct AssignStmtModel {
  // The name of the variable being assigned to.
  name: String,

  // The expression being assigned to the variable.
  expression: ExpressionModel,
}
impl AssignStmtModel {
  /** Create a new assignment statement. */
  pub(crate) fn new(name: String, expression: ExpressionModel) -> Self {
    AssignStmtModel { name, expression }
  }

  /** Get the name of the variable being assigned to. */
  pub(crate) fn name(&self) -> &str {
    &self.name
  }

  /** Get the expression being assigned to the variable. */
  pub(crate) fn expression(&self) -> &ExpressionModel {
    &self.expression
  }
}

/**
 * Represents an if-else statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct IfElseStmtModel {
  // The condition expression.
  condition: ExpressionModel,

  // The code block to execute if the condition is true.
  if_block: CodeBlockModel,

  // The code block to execute if the condition is false.
  else_block: Option<CodeBlockModel>,
}
impl IfElseStmtModel {
  /** Create a new if-else statement. */
  pub(crate) fn new(
    condition: ExpressionModel,
    if_block: CodeBlockModel,
    else_block: Option<CodeBlockModel>,
  ) -> Self {
    IfElseStmtModel {
      condition,
      if_block,
      else_block,
    }
  }

  /** Get the condition expression. */
  pub(crate) fn condition(&self) -> &ExpressionModel {
    &self.condition
  }

  /** Get the code block to execute if the condition is true. */
  pub(crate) fn if_block(&self) -> &CodeBlockModel {
    &self.if_block
  }

  /** Get the code block to execute if the condition is false. */
  pub(crate) fn else_block(&self) -> Option<&CodeBlockModel> {
    self.else_block.as_ref()
  }
}

/**
 * Represents an expression statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct ExprStmtModel {
  expression: ExpressionModel,
}
impl ExprStmtModel {
  /** Create a new expression statement. */
  pub(crate) fn new(expression: ExpressionModel) -> Self {
    ExprStmtModel { expression }
  }

  /** Get the expression. */
  pub(crate) fn expression(&self) -> &ExpressionModel {
    &self.expression
  }
}

/**
 * Represents a return statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct ReturnStmtModel {
  // The expression being returned.
  expression: Option<ExpressionModel>,
}
impl ReturnStmtModel {
  /** Create a new return statement. */
  pub(crate) fn new(expression: Option<ExpressionModel>) -> Self {
    ReturnStmtModel { expression }
  }

  /** Get the expression being returned. */
  pub(crate) fn expression(&self) -> Option<&ExpressionModel> {
    self.expression.as_ref()
  }
}
