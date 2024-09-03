use crate::{
  api::block_dims::BlockDims,
  model::CodeBlockModel,
};

/**
 * Models an entry point in a shader module.
 */
#[derive(Clone, Debug)]
pub(crate) struct EntryPointModel {
  // The name of the entry point.
  name: String,

  // The block dims of the entry point.
  block_dims: BlockDims,

  // The code block for the entry point.
  code_block: CodeBlockModel,
}
impl EntryPointModel {
  /** Create a new entry point. */
  pub(crate) fn new(
    name: String,
    block_dims: BlockDims,
    code_block: CodeBlockModel,
  ) -> EntryPointModel {
    EntryPointModel { name, block_dims, code_block }
  }

  /** Get the name of the entry point. */
  pub(crate) fn name(&self) -> &str {
    &self.name
  }

  /** Get the block dims of the entry point. */
  pub(crate) fn block_dims(&self) -> BlockDims {
    self.block_dims
  }

  /** Get the code block for the entry point. */
  pub(crate) fn code_block(&self) -> &CodeBlockModel {
    &self.code_block
  }
}
