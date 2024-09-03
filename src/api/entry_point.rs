use std::marker::PhantomData;
use crate::{
  api::data_type::EntryPointArgDataType,
  model::EntryPointModel,
};

/**
 * Represents a typed entry point within a shader.
 */
pub struct EntryPoint<T: EntryPointArgDataType> {
  _model: EntryPointModel,
  _phantom: PhantomData<T>,
}
impl<T: EntryPointArgDataType> EntryPoint<T> {
  /** Create a new entry point. */
  pub(crate) fn new(model: EntryPointModel) -> Self {
    EntryPoint { _model: model, _phantom: PhantomData }
  }
}
