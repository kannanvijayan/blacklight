/**
 * Buffer binding disposition trait.
 */
pub trait BufferDisposition {
  const REPR: BufferDispositionRepr;
}

pub struct BufferRead;
impl BufferDisposition for BufferRead {
  const REPR: BufferDispositionRepr = BufferDispositionRepr::Read;
}

pub struct BufferWrite;
impl BufferDisposition for BufferWrite {
  const REPR: BufferDispositionRepr = BufferDispositionRepr::Write;
}

pub struct BufferReadWrite;
impl BufferDisposition for BufferReadWrite {
  const REPR: BufferDispositionRepr = BufferDispositionRepr::ReadWrite;
}

/**
 * Runtime buffer disposition.
 */
#[derive(Clone, Copy, Debug)]
pub enum BufferDispositionRepr { Read, Write, ReadWrite }
impl BufferDispositionRepr {
  /** Get the string representation of the buffer disposition. */
  pub fn as_str(&self) -> &'static str {
    match self {
      BufferDispositionRepr::Read => "read",
      BufferDispositionRepr::Write => "write",
      BufferDispositionRepr::ReadWrite => "read_write",
    }
  }
}
