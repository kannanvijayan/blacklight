use std::marker::PhantomData;
use crate::{
  api::{
    data_type::{ ExprDataType, ProcResultType },
    handle::{ ExprHandle, LvalueHandle, VariableBindingHandle },
    variable_attributes::{
      VariableMutability,
      VariableRead,
      VariableReadWrite,
    },
  },
  data_type::HostShareableDataType,
  model::{
    AssignStmtModel,
    CodeBlockModel,
    ExprStmtModel,
    IdentifierModel,
    IfElseStmtModel,
    ReturnStmtModel,
    StatementModel,
    VarDeclStmtModel,
    VariableBindingDisposition,
    VariableBindingModel,
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

  fn build_sub_code_block<B>(&mut self, builder_func: B) -> CodeBlockModel
    where B: FnOnce(&mut CodeBlockBuilder<'sh, 'sh, ReturnT>),
  {
    let mut code_block_builder = CodeBlockBuilder::new();
    builder_func(&mut code_block_builder);
    code_block_builder.build()
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
  pub fn add_return_statement(&mut self, expr: ExprHandle<'cb, ReturnT>)
    where ReturnT: ExprDataType
  {
    let return_stmt_model = ReturnStmtModel::new(Some(expr.model));
    self.statements.push(StatementModel::Return(return_stmt_model));
  }

  /**
   * Add a new let or var declaration statement.
   * 
   * Returns an expression handle referencing the declared variable.
   *
   * Expressions used in an variable declaration statement _must_ be
   * constructed within the same block as the return statement itself,
   * as indicated by the lifetime constriant `cb`.
   */
  fn add_decl_statement<DT, MUT>(&mut self,
    binding_disposition: VariableBindingDisposition,
    name: &str,
    expr: ExprHandle<'cb, DT>,
  ) -> VariableBindingHandle<'cb, DT, MUT>
    where DT: ExprDataType,
          MUT: VariableMutability
  {
    let identifier_model = IdentifierModel::new(name);
    let var_binding_model = VariableBindingModel::new(
      identifier_model.clone(),
      binding_disposition,
      DT::repr(),
      Some(expr.model)
    );
    let var_decl_stmt_model = VarDeclStmtModel::new(var_binding_model);
    self.statements.push(StatementModel::VarDecl(var_decl_stmt_model));
    VariableBindingHandle::new(identifier_model)
  }

  /**
   * Add a new variable declaration statement.
   */
  pub fn add_var_decl_statement<DT>(&mut self,
    name: &str,
    expr: ExprHandle<'cb, DT>,
  ) -> VariableBindingHandle<'cb, DT, VariableReadWrite>
    where DT: ExprDataType
  {
    self.add_decl_statement(VariableBindingDisposition::Var, name, expr)
  }

  /**
   * Add a new let declaration statement.
   */
  pub fn add_let_decl_statement<DT>(&mut self,
    name: &str,
    expr: ExprHandle<'cb, DT>,
  ) -> VariableBindingHandle<'cb, DT, VariableRead>
    where DT: HostShareableDataType
  {
    self.add_decl_statement(VariableBindingDisposition::Let, name, expr)
  }

  /**
   * Add a new assignment statement.
   */
  pub fn add_assignment_statement<DT>(&mut self,
    lvalue: &LvalueHandle<'cb, DT>,
    expr: ExprHandle<'cb, DT>,
  )
    where DT: HostShareableDataType
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
    IfB: for <'if_cb> FnOnce(&mut CodeBlockBuilder<'if_cb, 'sh, ReturnT>),
    ElseB: for <'else_cb> FnOnce(&mut CodeBlockBuilder<'else_cb, 'sh, ReturnT>),
  {
    let if_block = self.build_sub_code_block(if_builder);
    let else_block = self.build_sub_code_block(else_builder);
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
    IfB: for <'if_cb> FnOnce(&mut CodeBlockBuilder<'if_cb, 'sh, ReturnT>)
  {
    let if_block = self.build_sub_code_block(if_builder);

    let if_else_stmt = IfElseStmtModel::new(condition.model, if_block, None);
    self.statements.push(StatementModel::IfElse(if_else_stmt));
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
