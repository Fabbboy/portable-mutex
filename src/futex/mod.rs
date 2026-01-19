#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
compile_error!("Futex is not implemented for this operating system");

pub mod bindings;

pub struct Futex;

impl Futex {
  pub unsafe fn wait(curr: *const u32, expected: u32) -> isize {
    #[cfg(target_os = "linux")]
    {
      unsafe { linux::Futex::wait(curr, expected) }
    }
  }

  pub unsafe fn wake(curr: *const u32, waiters: usize) -> isize {
    #[cfg(target_os = "linux")]
    {
      unsafe { linux::Futex::wake(curr, waiters) }
    }
  }
}
