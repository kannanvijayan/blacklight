use crate::{
  model::CodeBlockModel,
  util::Shared,
};

/**
 * Models an entry point in a shader module.
 */
#[derive(Clone, Debug)]
pub(crate) struct EntryPointModel {
  _shared: Shared<EntryPointModelShared>
}
impl EntryPointModel {
  /** Create a new entry point. */
  pub(crate) fn new(name: String, code_block: CodeBlockModel) -> EntryPointModel {
    EntryPointModel {
      _shared: Shared::new(EntryPointModelShared::new(name, code_block))
    }
  }
}

#[derive(Clone, Debug)]
struct EntryPointModelShared {
  // The name of the entry point.
  _name: String,

  // The code block for the entry point.
  _code_block: CodeBlockModel,
}
impl EntryPointModelShared {
  /** Create a new entry point. */
  pub(crate) fn new(name: String, code_block: CodeBlockModel) -> Self {
    EntryPointModelShared { _name: name, _code_block: code_block }
  }
}
