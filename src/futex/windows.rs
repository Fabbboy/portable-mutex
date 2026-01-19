#[cfg(not(target_os = "windows"))]
compile_error!("This file is only for Windows OS");

use windows_sys::Win32::System::Threading::{
  WaitOnAddress,
  WakeByAddressSingle,
};

pub struct Futex;

impl Futex {
  pub unsafe fn wait(curr: *const u32, expected: u32) -> isize {}
}
