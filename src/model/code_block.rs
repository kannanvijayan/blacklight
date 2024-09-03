use crate::model::StatementModel;

/**
 * Models a code block within a function or entry point.
 * Each function or entry point is associated with a single root-level
 * code block.
 */
#[derive(Clone, Debug)]
pub(crate) struct CodeBlockModel {
  statements: Vec<StatementModel>,
}
impl CodeBlockModel {
  /** Create a new code block.  */
  pub(crate) fn new(statements: Vec<StatementModel>) -> Self {
    CodeBlockModel { statements }
  }

  /** Get the statements in the code block. */
  pub(crate) fn statements(&self) -> &[StatementModel] {
    &self.statements
  }
}
