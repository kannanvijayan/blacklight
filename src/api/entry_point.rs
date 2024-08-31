use std::marker::PhantomData;
use crate::{
  api::data_type::ShEpArgDataType,
  model::EntryPointModel,
};

/**
 * Represents a typed entry point within a shader.
 */
pub struct EntryPoint<T: ShEpArgDataType> {
  _model: EntryPointModel,
  _phantom: PhantomData<T>,
}
impl<T: ShEpArgDataType> EntryPoint<T> {
  /** Create a new entry point. */
  pub(crate) fn new(model: EntryPointModel) -> Self {
    EntryPoint { _model: model, _phantom: PhantomData }
  }
}
