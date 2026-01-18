use core::sync::atomic::{
  AtomicU32,
  Ordering,
};

use crate::futex::Futex;

#[repr(u32)]
enum State {
  Unlocked,
  Locked,
  Contended,
}

pub struct RawMutex {
  value: AtomicU32,
}

impl RawMutex {
  pub const fn new() -> Self {
    RawMutex {
      value: AtomicU32::new(State::Unlocked as u32),
    }
  }

  #[inline]
  pub fn try_lock(&self) -> bool {
    let prev = self.value.compare_exchange(
      State::Unlocked as u32,
      State::Locked as u32,
      Ordering::Acquire,
      Ordering::Relaxed,
    );

    prev.is_ok()
  }

  pub fn lock(&self) {
    if self.try_lock() {
      return;
    }

    loop {
      if self.value.load(Ordering::Acquire) == State::Locked as u32 {
        let _ = self.value.compare_exchange(
          State::Locked as u32,
          State::Contended as u32,
          Ordering::Acquire,
          Ordering::Relaxed,
        );
      }

      if self.try_lock() {
        return;
      }

      while self.value.load(Ordering::Acquire) == State::Contended as u32 {
        unsafe {
          Futex::wait(self.value.as_ptr(), State::Contended as u32);
        }
      }
    }
  }

  #[inline]
  pub fn unlock(&self) {
    let prev = self.value.swap(State::Unlocked as u32, Ordering::Release);

    if prev != State::Contended as u32 {
      return;
    }

    unsafe {
      Futex::wake(self.value.as_ptr(), 1);
    }
  }
}
