use crate::model::{ DataTypeCollector, StatementModel };

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

  /** Collect struct data types reference by this buffer into a vector. */
  pub(crate) fn collect_struct_data_types_into(&self,
    collector: &mut DataTypeCollector,
  ) {
    for statement in &self.statements {
      statement.collect_struct_data_types_into(collector);
    }
  }
}
