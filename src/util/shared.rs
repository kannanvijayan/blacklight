use std::{
  borrow::Borrow,
  fmt::Debug,
  ops::Deref,
  rc::Rc,
};

/**
 * A wrapper around refcount semantics to allow for easy abstraction
 * around Rc vs Arc later.
 */
pub struct Shared<T: 'static> {
  inner: Rc<T>,
}
impl<T: 'static> Shared<T> {
  /** Create a new shared object. */
  pub fn new(inner: T) -> Self {
    Self { inner: Rc::new(inner) }
  }
}
impl<T: 'static> AsRef<T> for Shared<T> {
  fn as_ref(&self) -> &T {
    &self.inner
  }
}
impl<T: 'static> Clone for Shared<T> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone(),
    }
  }
}
impl<T: 'static> Borrow<T> for Shared<T> {
  fn borrow(&self) -> &T {
    &self.inner
  }
}
impl<T: 'static> Deref for Shared<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}
impl<T: 'static> Debug for Shared<T> where T: Debug
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Shared({:?})", self.inner)
  }
}
