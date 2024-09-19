use std::marker::PhantomData;
use crate::{
  api::data_type::ExprDataType,
  model::LvalueModel,
};

/**
 * A handle identifying an lvalue within an active shader declaration.
 */
pub struct LvalueHandle<'a, DT: ExprDataType> {
  model: LvalueModel,
  _phantom: PhantomData<&'a DT>,
}
impl<'a, DT: ExprDataType> LvalueHandle<'a, DT> {
  /** Create a new lvalue handle for the given model. */
  pub(crate) fn new(model: LvalueModel) -> Self {
    LvalueHandle { model, _phantom: PhantomData }
  }

  /** Get the model. */
  pub(crate) fn model(&self) -> &LvalueModel {
    &self.model
  }
}
impl<'a, DT: ExprDataType> Clone for LvalueHandle<'a, DT> {
  fn clone(&self) -> Self {
    LvalueHandle { model: self.model.clone(), _phantom: PhantomData }
  }
}
