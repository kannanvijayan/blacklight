use crate::{
  api::data_type::DataTypeRepr,
  model::{
    CodeBlockModel,
    DataTypeCollector,
    ExpressionModel,
    IdentifierModel,
    LvalueModel,
  },
};

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
impl StatementModel {
  /** Collect struct data types reference by this buffer into a vector. */
  pub(crate) fn collect_struct_data_types_into(&self,
    collector: &mut DataTypeCollector,
  ) {
    match self {
      StatementModel::VarDecl(var_decl_stmt) => {
        var_decl_stmt.expression.collect_struct_data_types_into(collector);
      },
      StatementModel::Assign(assign_stmt) => {
        assign_stmt.expression.collect_struct_data_types_into(collector);
      },
      StatementModel::IfElse(if_else_stmt) => {
        if_else_stmt.condition.collect_struct_data_types_into(collector);
        if_else_stmt.if_block.collect_struct_data_types_into(collector);
        if let Some(else_block) = &if_else_stmt.else_block {
          else_block.collect_struct_data_types_into(collector);
        }
      },
      StatementModel::Expr(expr_stmt) => {
        expr_stmt.expression.collect_struct_data_types_into(collector);
      },
      StatementModel::Return(return_stmt) => {
        if let Some(expression) = &return_stmt.expression {
          expression.collect_struct_data_types_into(collector);
        }
      }
    }
  }
}

/**
 * Represents a variable declaration statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct VarDeclStmtModel {
  // The name of the variable being declared.
  name: IdentifierModel,

  // The expression being assigned to the variable.
  expression: Box<ExpressionModel>,
}
impl VarDeclStmtModel {
  /** Create a new variable declaration statement. */
  pub(crate) fn new(name: IdentifierModel, expression: Box<ExpressionModel>) -> Self {
    VarDeclStmtModel { name, expression }
  }

  /** Get the name of the variable being declared. */
  pub(crate) fn name(&self) -> &IdentifierModel {
    &self.name
  }

  /** Get the expression being assigned to the variable. */
  pub(crate) fn expression(&self) -> &Box<ExpressionModel> {
    &self.expression
  }

  /** Get the data type for this statement model. */
  pub(crate) fn data_type(&self) -> DataTypeRepr {
    self.expression.data_type()
  }
}

/**
 * Represents an assignment statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct AssignStmtModel {
  // The name of the variable being assigned to.
  target: LvalueModel,

  // The expression being assigned to the variable.
  expression: Box<ExpressionModel>,
}
impl AssignStmtModel {
  /** Create a new assignment statement. */
  pub(crate) fn new(target: LvalueModel, expression: Box<ExpressionModel>) -> Self {
    AssignStmtModel { target, expression }
  }

  /** Get the name of the variable being assigned to. */
  pub(crate) fn target(&self) -> &LvalueModel {
    &self.target
  }

  /** Get the expression being assigned to the variable. */
  pub(crate) fn expression(&self) -> &Box<ExpressionModel> {
    &self.expression
  }
}

/**
 * Represents an if-else statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct IfElseStmtModel {
  // The condition expression.
  condition: Box<ExpressionModel>,

  // The code block to execute if the condition is true.
  if_block: CodeBlockModel,

  // The code block to execute if the condition is false.
  else_block: Option<CodeBlockModel>,
}
impl IfElseStmtModel {
  /** Create a new if-else statement. */
  pub(crate) fn new(
    condition: Box<ExpressionModel>,
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
  pub(crate) fn condition(&self) -> &Box<ExpressionModel> {
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
  expression: Box<ExpressionModel>,
}
impl ExprStmtModel {
  /** Create a new expression statement. */
  pub(crate) fn new(expression: Box<ExpressionModel>) -> Self {
    ExprStmtModel { expression }
  }

  /** Get the expression. */
  pub(crate) fn expression(&self) -> &Box<ExpressionModel> {
    &self.expression
  }
}

/**
 * Represents a return statement.
 */
#[derive(Clone, Debug)]
pub(crate) struct ReturnStmtModel {
  // The expression being returned.
  expression: Option<Box<ExpressionModel>>,
}
impl ReturnStmtModel {
  /** Create a new return statement. */
  pub(crate) fn new(expression: Option<Box<ExpressionModel>>) -> Self {
    ReturnStmtModel { expression }
  }

  /** Get the expression being returned. */
  pub(crate) fn expression(&self) -> Option<&Box<ExpressionModel>> {
    self.expression.as_ref()
  }
}
