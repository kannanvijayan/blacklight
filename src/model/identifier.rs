use crate::util::Shared;

/**
 * Represents an identifier used in syntax.
 */
#[derive(Clone, Debug)]
pub struct IdentifierModel(Shared<String>);
impl IdentifierModel {
  /** Create a new identifier model. */
  pub fn new(name: &str) -> Self {
    IdentifierModel(Shared::new(name.to_string()))
  }

  /** Get the name of the identifier. */
  pub fn as_str(&self) -> &str {
    &self.0
  }
}
