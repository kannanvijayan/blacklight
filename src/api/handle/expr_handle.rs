use std::{
  marker::PhantomData,
  ops,
};
use crate::{
  api::data_type::{ ExprDataType, ExprNumericDataType },
  model::{ BinOp, BinOpExprModel, CmpOp, CmpOpExprModel, ExpressionModel },
};

/**
 * A typed, lifetime-aware handle to an expression within a shader.
 */
pub struct ExprHandle<'cb, DT: ExprDataType> {
  pub(crate) model: ExpressionModel,
  _phantom: PhantomData<&'cb DT>,
}
impl<'cb, DT: ExprDataType> ExprHandle<'cb, DT> {
  /** Create a new expression handle for the given expression model. */
  pub(crate) fn new(model: ExpressionModel) -> Self {
    ExprHandle { model, _phantom: PhantomData }
  }

  /** Build a comparison expression. */
  pub fn cmp(&self, other: &Self, cmp_op: CmpOp) -> ExprHandle<'cb, bool> {
    let cmp_op_expr_model =
      CmpOpExprModel::new(self.model.clone(), other.model.clone(), cmp_op);
    let model = ExpressionModel::CmpOp(cmp_op_expr_model);
    ExprHandle::new(model)
  }

  /** Build an equality `==` expression. */
  pub fn eq(&self, other: &Self) -> ExprHandle<'cb, bool> {
    self.cmp(other, CmpOp::Eq)
  }

  /** Build an non-equality `!=` expression. */
  pub fn ne(&self, other: &Self) -> ExprHandle<'cb, bool> {
    self.cmp(other, CmpOp::Ne)
  }

  /** Build a less-than `<` expression. */
  pub fn lt(&self, other: &Self) -> ExprHandle<'cb, bool> {
    self.cmp(other, CmpOp::Lt)
  }

  /** Build a less-than-or-equal-to `<=` expression. */
  pub fn le(&self, other: &Self) -> ExprHandle<'cb, bool> {
    self.cmp(other, CmpOp::Le)
  }

  /** Build a greater-than `>` expression. */
  pub fn gt(&self, other: &Self) -> ExprHandle<'cb, bool> {
    self.cmp(other, CmpOp::Gt)
  }

  /** Build a greater-than-or-equal-to `>=` expression. */
  pub fn ge(&self, other: &Self) -> ExprHandle<'cb, bool> {
    self.cmp(other, CmpOp::Ge)
  }
}
impl<'cb, DT: ExprDataType> Clone for ExprHandle<'cb, DT> {
  fn clone(&self) -> Self {
    ExprHandle { model: self.model.clone(), _phantom: PhantomData }
  }
}

// Generin binop helper.
fn make_binop<'cb, DT>(
  lhs: ExprHandle<'cb, DT>,
  rhs: ExprHandle<'cb, DT>,
  binop: BinOp
) -> ExprHandle<'cb, DT>
  where DT: ExprNumericDataType
{
  let binop_expr_model = BinOpExprModel::new(
    lhs.model.clone(),
    rhs.model.clone(),
    binop,
  );
  ExprHandle::new(ExpressionModel::BinOp(binop_expr_model))
}

// Macro to generate binary operator implementations.
macro_rules! impl_binop {
  ($class:ident, $op:ident, $binop:expr) => {
    impl<'cb, DT> ops::$class for ExprHandle<'cb, DT>
      where DT: ExprNumericDataType
    {
      type Output = ExprHandle<'cb, DT>;
      fn $op(self, other: Self) -> Self::Output {
        make_binop(self, other, $binop)
      }
    }
  };
}

impl_binop!(Add, add, BinOp::Add);
impl_binop!(Sub, sub, BinOp::Sub);
impl_binop!(Mul, mul, BinOp::Mul);
impl_binop!(Div, div, BinOp::Div);
impl_binop!(Rem, rem, BinOp::Rem);
