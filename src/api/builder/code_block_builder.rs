use std::marker::PhantomData;
use crate::{
  api::{
    handle::{ ExprHandle, LvalueHandle },
    data_type::{ ExprDataType, LiteralDataType, ProcResultType, VarDataType }
  },
  model::{
    AssignStmtModel,
    CodeBlockModel,
    ExprStmtModel,
    ExpressionModel,
    IfElseStmtModel,
    LiteralExprModel,
    LvalueModel,
    ReturnStmtModel,
    StatementModel,
    VarDeclStmtModel,
  },
};


/**
 * A builder helper for defining shader entrypoints.
 */
pub struct CodeBlockBuilder<'cb, 'sh: 'cb, ReturnT>
  where ReturnT: ProcResultType
{
  statements: Vec<StatementModel>,
  _phantom: PhantomData<&'cb &'sh ReturnT>,
}
impl<'cb, 'sh: 'cb, ReturnT> CodeBlockBuilder<'cb, 'sh, ReturnT>
  where ReturnT: ProcResultType
{
  /** Create a new shader entrypoint builder for the given shader builder. */
  pub(crate) fn new() -> Self {
    CodeBlockBuilder {
      statements: Vec::new(),
      _phantom: PhantomData,
    }
  }

  /** Build an entrypoint from the definition provided. */
  pub(crate) fn build(self) -> CodeBlockModel {
    CodeBlockModel::new(self.statements)
  }

  /**
   * Add a new expressions statement.
   *
   * Expressions used in an expression statement _must_ be constructed within
   * the same block as the return statement itself, as indicated by the
   * lifetime constriant `cb`.
   */
  pub fn add_expr_statement<DT>(&mut self, expr: ExprHandle<'cb, DT>)
    where DT: ExprDataType,
  {
    let expr_stmt_model = ExprStmtModel::new(expr.model);
    self.statements.push(StatementModel::Expr(expr_stmt_model));
  }

  /**
   * Add a return statement.
   * Only available in codeblocks in functions with a non-void return.
   * See `add_bare_return_statement` for use in functions returning void.
   *
   * Expressions used in a return statement _must_ be constructed within
   * the same block as the return statement itself, as indicated by the
   * lifetime constriant `cb`.
   */
  pub fn add_return_statement<DT>(&mut self, expr: ExprHandle<'cb, ReturnT>)
    where ReturnT: ExprDataType
  {
    let return_stmt_model = ReturnStmtModel::new(Some(expr.model));
    self.statements.push(StatementModel::Return(return_stmt_model));
  }

  /**
   * Add a new variable declaration statement.
   * 
   * Returns an expression handle referencing the declared variable.
   *
   * Expressions used in an variable declaration statement _must_ be
   * constructed within the same block as the return statement itself,
   * as indicated by the lifetime constriant `cb`.
   */
  pub fn add_var_decl_statement<DT>(&mut self,
    name: &str,
    expr: ExprHandle<'cb, DT>,
  ) -> LvalueHandle<'cb, DT>
    where DT: VarDataType
  {
    let var_decl_stmt_model = VarDeclStmtModel::new(name.to_string(), expr.model);
    self.statements.push(StatementModel::VarDecl(var_decl_stmt_model));

    let lvalue_var_model = LvalueModel::new_variable(name.to_string());
    LvalueHandle::new(lvalue_var_model)
  }

  /**
   * Add a new assignment statement.
   */
  pub fn add_assignment_statement<DT>(&mut self,
    lvalue: &LvalueHandle<'cb, DT>,
    expr: ExprHandle<'cb, DT>,
  )
    where DT: VarDataType
  {
    let assign_stmt_model = AssignStmtModel::new(
      lvalue.model().clone(),
      expr.model
    );
    self.statements.push(StatementModel::Assign(assign_stmt_model));
  }

  /**
   * Add a new if-else statement.
   */
  pub fn add_if_else_statement<IfB, ElseB>(&mut self,
    condition: ExprHandle<'cb, bool>,
    if_builder: IfB,
    else_builder: ElseB,
  ) where
    IfB: for <'if_cb> FnOnce(&mut CodeBlockBuilder<'if_cb, 'sh, ()>),
    ElseB: for <'else_cb> FnOnce(&mut CodeBlockBuilder<'else_cb, 'sh, ()>),
  {
    let mut if_block_builder = CodeBlockBuilder::new();
    if_builder(&mut if_block_builder);
    let if_block = if_block_builder.build();

    let mut else_block_builder = CodeBlockBuilder::new();
    else_builder(&mut else_block_builder);
    let else_block = else_block_builder.build();

    let if_else_stmt =
      IfElseStmtModel::new(condition.model, if_block, Some(else_block));
    self.statements.push(StatementModel::IfElse(if_else_stmt));
  }

  /**
   * Add a new if statement.
   */
  pub fn add_if_statement<IfB>(&mut self,
    condition: ExprHandle<'cb, bool>,
    if_builder: IfB,
  ) where
    IfB: FnOnce(&mut CodeBlockBuilder<'cb, 'sh, ()>)
  {
    let mut if_block_builder = CodeBlockBuilder::new();
    if_builder(&mut if_block_builder);
    let if_block = if_block_builder.build();

    let if_else_stmt = IfElseStmtModel::new(condition.model, if_block, None);
    self.statements.push(StatementModel::IfElse(if_else_stmt));
  }

  /**
   * Create a new literal expression.
   */
  pub fn literal_expr<DT>(&self, value: DT) -> ExprHandle<'cb, DT>
    where DT: LiteralDataType
  {
    let literal_value = value.to_sh_literal_data_value();
    let literal_expr_model = LiteralExprModel::new(literal_value);
    ExprHandle::new(ExpressionModel::Literal(literal_expr_model))
  }
}

/**
 * Implementations for the CodeBlockBuilder on functions returning void.
 */
impl<'cb, 'sh: 'cb> CodeBlockBuilder<'cb, 'sh, ()> {
  /** Add a new bare return statement. */
  pub fn add_bare_return_statement(&mut self) {
    let return_stmt_model = ReturnStmtModel::new(None);
    self.statements.push(StatementModel::Return(return_stmt_model));
  }
}
