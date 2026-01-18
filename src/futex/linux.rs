#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_64;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
compile_error!("Futex wait is not implemented for this architecture");

pub struct Futex;

impl Futex {
  #[inline(always)]
  #[cfg(target_arch = "x86")]
  pub unsafe fn wait32(curr: *const u32, expected: u32) -> isize {
    use crate::futex::bindings::{
      __NR_futex as SYS_futex,
      FUTEX_WAIT,
    };

    unsafe {
      x86_64::syscall6(
        SYS_futex as isize,
        curr as usize,
        FUTEX_WAIT as usize,
        expected as usize,
        0,
        0,
        0,
      )
    }
  }

  #[inline(always)]
  #[cfg(target_arch = "x86_64")]
  pub unsafe fn wait64(curr: *const u32, expected: u32) -> isize {
    use crate::futex::bindings::{
      __NR_futex as SYS_futex,
      FUTEX_WAIT,
    };

    unsafe {
      x86_64::syscall6(
        SYS_futex as isize,
        curr as usize,
        FUTEX_WAIT as usize,
        expected as usize,
        0,
        0,
        0,
      )
    }
  }

  #[inline(always)]
  pub unsafe fn wait(curr: *const u32, expected: u32) -> isize {
    #[cfg(target_arch = "x86")]
    {
      unsafe { Self::wait32(curr, expected) }
    }

    #[cfg(target_arch = "x86_64")]
    {
      unsafe { Self::wait64(curr, expected) }
    }
  }

  #[inline(always)]
  #[cfg(target_arch = "x86")]
  pub unsafe fn wake32(curr: *const u32, waiters: usize) -> isize {
    use crate::futex::bindings::{
      __NR_futex as SYS_futex,
      FUTEX_WAKE,
    };

    unsafe {
      x86_64::syscall6(
        SYS_futex as isize,
        curr as usize,
        FUTEX_WAKE as usize,
        waiters,
        0,
        0,
        0,
      )
    }
  }

  #[inline(always)]
  #[cfg(target_arch = "x86_64")]
  pub unsafe fn wake64(curr: *const u32, waiters: usize) -> isize {
    use crate::futex::bindings::{
      __NR_futex as SYS_futex,
      FUTEX_WAKE,
    };

    unsafe {
      x86_64::syscall6(
        SYS_futex as isize,
        curr as usize,
        FUTEX_WAKE as usize,
        waiters,
        0,
        0,
        0,
      )
    }
  }

  #[inline(always)]
  pub unsafe fn wake(curr: *const u32, waiters: usize) -> isize {
    #[cfg(target_arch = "x86")]
    {
      unsafe { Self::wake32(curr, waiters) }
    }

    #[cfg(target_arch = "x86_64")]
    {
      unsafe { Self::wake64(curr, waiters) }
    }
  }
}
