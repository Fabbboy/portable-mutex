use core::cell::UnsafeCell;

use crate::{
  guard::MutexGuard,
  raw::RawMutex,
};

pub struct Mutex<T>
where
  T: ?Sized,
{
  pub(crate) raw: RawMutex,
  pub(crate) data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: ?Sized + Send {}
unsafe impl<T> Send for Mutex<T> where T: ?Sized + Send {}

impl<T> Mutex<T> {
  pub const fn new(data: T) -> Self {
    Mutex {
      raw: RawMutex::new(),
      data: UnsafeCell::new(data),
    }
  }

  pub fn lock(&self) -> MutexGuard<'_, T> {
    self.raw.lock();

    MutexGuard::new(self)
  }
}
