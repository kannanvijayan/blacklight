use std::marker::PhantomData;
use crate::{
  api::{
    handle::ExprHandle,
    data_type::VarDataType,
  },
  model::{ ExpressionModel, IdentifierExprModel },
};

/**
 * A handle identifying an lvalue within an active shader declaration.
 */
pub struct LvalueHandle<'a, DT: VarDataType> {
  pub(crate) name: String,
  _phantom: PhantomData<&'a DT>,
}
impl<'a, DT: VarDataType> LvalueHandle<'a, DT> {
  /** Create a new lvalue handle for the given name. */
  pub(crate) fn new(name: String) -> Self {
    LvalueHandle { name, _phantom: PhantomData }
  }

  /** Convert the LvalueHandle to an ExprHandle to use as a read expression. */
  pub fn read(&self) -> ExprHandle<'a, DT> {
    let identifier_expr_model = IdentifierExprModel::new(self.name.clone());
    ExprHandle::new(ExpressionModel::Identifier(identifier_expr_model))
  }
}
impl<'a, DT: VarDataType> Clone for LvalueHandle<'a, DT> {
  fn clone(&self) -> Self {
    LvalueHandle { name: self.name.clone(), _phantom: PhantomData }
  }
}
