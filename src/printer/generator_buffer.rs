/**
 * Provides a simple code-generation API for printing text in a structured
 * way to a buffer.
 */
pub(crate) struct GeneratorBuffer {
  /** The buffer to write to. */
  buffer: Vec<String>,

  /** The current indent level. */
  indent: usize,
}
impl GeneratorBuffer {
  /** Create a new buffer. */
  pub(crate) fn new() -> Self {
    Self { buffer: Vec::new(), indent: 0 }
  }

  /** Evaluate the given procedure with an increased indent level. */
  pub(crate) fn with_indent<F>(&mut self, f: F)
  where
    F: FnOnce(&mut Self),
  {
    self.indent += 1;
    f(self);
    self.indent -= 1;
  }

  /** Write a number of spaces equal to the indent level. */
  fn write_indent(&mut self) {
    self.buffer.push("  ".repeat(self.indent));
  }

  /** Write the start of a line to the buffer. */
  pub(crate) fn write_start(&mut self, start: impl AsRef<str>) {
    self.write_indent();
    self.buffer.push(start.as_ref().to_string());
  }

  /** Write a full line to the buffer. */
  pub(crate) fn write_line(&mut self, line: impl AsRef<str>) {
    self.write_indent();
    self.buffer.push(line.as_ref().to_string());
    self.newline();
  }

  /** Write the end of a line to the buffer. */
  pub(crate) fn write_end(&mut self, line: impl AsRef<str>) {
    self.buffer.push(line.as_ref().to_string());
    self.newline();
  }

  /** Write text to the buffer without a newline or indent. */
  pub(crate) fn write(&mut self, text: impl AsRef<str>) {
    self.buffer.push(text.as_ref().to_string());
  }

  /** Write a newline to the buffer. */
  pub(crate) fn newline(&mut self) {
    self.buffer.push("\n".to_string());
  }

  /** Get the buffer as a string. */
  pub(crate) fn to_string(&self) -> String {
    self.buffer.join("")
  }
}
