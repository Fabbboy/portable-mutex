use core::{
  cell::Cell,
  marker::PhantomData,
  ops::{
    Deref,
    DerefMut,
  },
};

use crate::mutex::Mutex;

pub struct MutexGuard<'m, T>
where
  T: ?Sized,
{
  mutex: &'m Mutex<T>,
  _no_send: PhantomData<Cell<()>>,
}

impl<'m, T> MutexGuard<'m, T>
where
  T: ?Sized,
{
  #[inline]
  pub(crate) fn new(mutex: &'m Mutex<T>) -> Self {
    MutexGuard {
      mutex,
      _no_send: PhantomData,
    }
  }
}

impl<'m, T> Deref for MutexGuard<'m, T>
where
  T: ?Sized,
{
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    unsafe { &*self.mutex.data.get() }
  }
}

impl<'m, T> DerefMut for MutexGuard<'m, T>
where
  T: ?Sized,
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.mutex.data.get() }
  }
}

impl<'m, T> Drop for MutexGuard<'m, T>
where
  T: ?Sized,
{
  #[inline]
  fn drop(&mut self) {
    self.mutex.raw.unlock();
  }
}
