/**
 * Buffer binding disposition trait.
 */
pub trait VariableMutability {
  const REPR: VariableMutabilityRepr;
}

pub struct VariableRead;
impl VariableMutability for VariableRead {
  const REPR: VariableMutabilityRepr = VariableMutabilityRepr::Read;
}

pub struct VariableReadWrite;
impl VariableMutability for VariableReadWrite {
  const REPR: VariableMutabilityRepr = VariableMutabilityRepr::ReadWrite;
}

/**
 * Runtime buffer disposition.
 */
#[derive(Clone, Copy, Debug)]
pub enum VariableMutabilityRepr { Read, ReadWrite }
impl VariableMutabilityRepr {
  /** Get the string representation of the buffer disposition. */
  pub fn as_str(&self) -> &'static str {
    match self {
      VariableMutabilityRepr::Read => "read",
      VariableMutabilityRepr::ReadWrite => "read_write",
    }
  }
}
