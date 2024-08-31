use std::{
  fmt::Debug,
  rc::Rc,
};

/**
 * A wrapper around refcount semantics to allow for easy abstraction
 * around Rc vs Arc later.
 */
pub struct Shared<T> {
  inner: Rc<T>,
}
impl<T> Shared<T> {
  /** Create a new shared object. */
  pub fn new(inner: T) -> Self {
    Self { inner: Rc::new(inner) }
  }
}
impl<T> AsRef<T> for Shared<T> {
  fn as_ref(&self) -> &T {
    &self.inner
  }
}
impl<T> Clone for Shared<T> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone(),
    }
  }
}
impl<T> Debug for Shared<T> where T: Debug
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Shared({:?})", self.inner)
  }
}
