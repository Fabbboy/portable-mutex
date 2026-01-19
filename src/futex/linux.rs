#[cfg(not(any(
  target_arch = "x86",
  target_arch = "x86_64",
  target_arch = "arm",
  target_arch = "aarch64"
)))]
compile_error!("Futex wait is not implemented for this architecture");

#[cfg(any(
  target_arch = "x86",
  target_arch = "x86_64",
  target_arch = "arm",
  target_arch = "aarch64"
))]
use crate::futex::bindings::{
  __NR_futex as SYS_futex,
  FUTEX_WAIT,
  FUTEX_WAKE,
};

pub struct Futex;

impl Futex {
  #[inline(always)]
  #[cfg(target_arch = "x86")]
  pub unsafe fn wait_x86_32(curr: *const u32, expected: u32) -> isize {
    let ret: isize;
    unsafe {
      core::arch::asm!(
        "int 0x80",
        inlateout("eax") SYS_futex as isize => ret,
        in("ebx") curr as usize,
        in("ecx") FUTEX_WAIT as usize,
        in("edx") expected as usize,
        in("esi") 0,
        in("edi") 0,
        in("ebp") 0,
        options(nostack),
      );
    }
    ret
  }

  #[inline(always)]
  #[cfg(target_arch = "x86_64")]
  pub unsafe fn wait_x86_64(curr: *const u32, expected: u32) -> isize {
    let ret: isize;
    unsafe {
      core::arch::asm!(
        "syscall",
        inlateout("rax") SYS_futex as isize => ret,
        in("rdi") curr as usize,
        in("rsi") FUTEX_WAIT as usize,
        in("rdx") expected as usize,
        in("r10") 0,
        in("r8") 0,
        in("r9") 0,
        clobber_abi("sysv64"),
      );
    }
    ret
  }

  #[inline(always)]
  #[cfg(target_arch = "arm")]
  pub unsafe fn wait_arm32(curr: *const u32, expected: u32) -> isize {
    let ret: isize;
    unsafe {
      core::arch::asm!(
        "svc 0",
        inlateout("r0") curr as usize => ret,
        in("r1") FUTEX_WAIT as usize,
        in("r2") expected as usize,
        in("r3") 0,
        in("r7") SYS_futex as usize,
        options(nostack),
      );
    }
    ret
  }

  #[inline(always)]
  #[cfg(target_arch = "aarch64")]
  pub unsafe fn wait_arm64(curr: *const u32, expected: u32) -> isize {
    let ret: isize;
    unsafe {
      core::arch::asm!(
        "svc 0",
        inlateout("x0") curr as usize => ret,
        in("x1") FUTEX_WAIT as usize,
        in("x2") expected as usize,
        in("x3") 0,
        in("x8") SYS_futex as usize,
        options(nostack),
      );
    }
    ret
  }

  #[inline(always)]
  pub unsafe fn wait(curr: *const u32, expected: u32) -> isize {
    #[cfg(target_arch = "x86")]
    {
      unsafe { Self::wait_x86_32(curr, expected) }
    }

    #[cfg(target_arch = "x86_64")]
    {
      unsafe { Self::wait_x86_64(curr, expected) }
    }

    #[cfg(target_arch = "arm")]
    {
      unsafe { Self::wait_arm32(curr, expected) }
    }

    #[cfg(target_arch = "aarch64")]
    {
      unsafe { Self::wait_arm64(curr, expected) }
    }
  }

  #[inline(always)]
  #[cfg(target_arch = "x86")]
  pub unsafe fn wake_x86_32(curr: *const u32, waiters: usize) -> isize {
    let ret: isize;
    unsafe {
      core::arch::asm!(
        "int 0x80",
        inlateout("eax") SYS_futex as isize => ret,
        in("ebx") curr as usize,
        in("ecx") FUTEX_WAKE as usize,
        in("edx") waiters,
        in("esi") 0,
        in("edi") 0,
        in("ebp") 0,
        options(nostack),
      );
    }
    ret
  }

  #[inline(always)]
  #[cfg(target_arch = "x86_64")]
  pub unsafe fn wake_x86_64(curr: *const u32, waiters: usize) -> isize {
    let ret: isize;
    unsafe {
      core::arch::asm!(
        "syscall",
        inlateout("rax") SYS_futex as isize => ret,
        in("rdi") curr as usize,
        in("rsi") FUTEX_WAKE as usize,
        in("rdx") waiters,
        in("r10") 0,
        in("r8") 0,
        in("r9") 0,
        clobber_abi("sysv64"),
        options(nostack),
      );
    }
    ret
  }

  #[inline(always)]
  #[cfg(target_arch = "arm")]
  pub unsafe fn wake_arm32(curr: *const u32, waiters: usize) -> isize {
    let ret: isize;
    unsafe {
      core::arch::asm!(
        "svc 0",
        inlateout("r0") curr as usize => ret,
        in("r1") FUTEX_WAKE as usize,
        in("r2") waiters,
        in("r3") 0,
        in("r7") SYS_futex as usize,
        options(nostack),
      );
    }
    ret
  }

  #[inline(always)]
  #[cfg(target_arch = "aarch64")]
  pub unsafe fn wake_arm64(curr: *const u32, waiters: usize) -> isize {
    let ret: isize;
    unsafe {
      core::arch::asm!(
        "svc 0",
        inlateout("x0") curr as usize => ret,
        in("x1") FUTEX_WAKE as usize,
        in("x2") waiters,
        in("x3") 0,
        in("x8") SYS_futex as usize,
        options(nostack),
      );
    }
    ret
  }

  #[inline(always)]
  pub unsafe fn wake(curr: *const u32, waiters: usize) -> isize {
    #[cfg(target_arch = "x86")]
    unsafe {
      Self::wake_x86_32(curr, waiters)
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
      Self::wake_x86_64(curr, waiters)
    }

    #[cfg(target_arch = "arm")]
    unsafe {
      Self::wake_arm32(curr, waiters)
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
      Self::wake_arm64(curr, waiters)
    }
  }
}
