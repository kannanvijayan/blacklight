use std::marker::PhantomData;
use crate::{
  api::{
    handle::ExprHandle,
    data_type::VarDataType,
  },
  model::LvalueModel,
};

/**
 * A handle identifying an lvalue within an active shader declaration.
 */
pub struct LvalueHandle<'a, DT: VarDataType> {
  model: LvalueModel,
  _phantom: PhantomData<&'a DT>,
}
impl<'a, DT: VarDataType> LvalueHandle<'a, DT> {
  /** Create a new lvalue handle for the given model. */
  pub(crate) fn new(model: LvalueModel) -> Self {
    LvalueHandle { model, _phantom: PhantomData }
  }

  /** Get the model. */
  pub(crate) fn model(&self) -> &LvalueModel {
    &self.model
  }

  /**
   * Convert the LvalueHandle to an ExprHandle to use as a read expression.
   *
   * This can happen in any sub-lifetime of the lifetime of the LvalueHandle.
   */
  pub fn read<'cb>(&self) -> ExprHandle<'cb, DT>
    where 'a: 'cb
  {
    ExprHandle::new(self.model.read_expr())
  }
}
impl<'a, DT: VarDataType> Clone for LvalueHandle<'a, DT> {
  fn clone(&self) -> Self {
    LvalueHandle { model: self.model.clone(), _phantom: PhantomData }
  }
}
