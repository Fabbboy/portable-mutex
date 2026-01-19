#[cfg(not(target_os = "windows"))]
compile_error!("This file is only for Windows OS");

use windows_sys::Win32::System::Threading::{
  WaitOnAddress,
  WakeByAddressSingle,
};

pub struct Futex;

impl Futex {
  pub unsafe fn wait(curr: *const u32, expected: u32) -> isize {
    unsafe {
      WaitOnAddress(
        curr as *const _,
        &expected as *const _ as *const _,
        core::mem::size_of::<u32>(),
        u32::MAX,
      );
    }
    0
  }

  pub unsafe fn wake(curr: *const u32, _waiters: usize) -> isize {
    unsafe {
      WakeByAddressSingle(curr as *const _);
    }
    0
  }
}
