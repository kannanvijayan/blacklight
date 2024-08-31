use std::marker::PhantomData;
use crate::{
  api::data_type::ShExprDataType,
  model::ExpressionModel,
};

/**
 * A typed, lifetime-aware handle to an expression within a shader.
 */
pub struct ExprHandle<'a, DT: ShExprDataType> {
  pub(crate) model: ExpressionModel,
  _phantom: PhantomData<&'a DT>,
}
impl<'a, DT: ShExprDataType> ExprHandle<'a, DT> {
  /** Create a new expression handle for the given expression model. */
  pub(crate) fn new(model: ExpressionModel) -> Self {
    ExprHandle { model, _phantom: PhantomData }
  }
}
impl<'a, DT: ShExprDataType> Clone for ExprHandle<'a, DT> {
  fn clone(&self) -> Self {
    ExprHandle { model: self.model.clone(), _phantom: PhantomData }
  }
}
