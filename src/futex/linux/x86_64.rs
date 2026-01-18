#[inline(always)]
#[cfg(target_arch = "x86")]
pub unsafe fn syscall6(num: isize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize) -> isize {
  let ret: isize;

  unsafe {
    core::arch::asm!(
        "int 0x80",
        inlateout("eax") num => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        in("edx") arg3,
        in("esi") arg4,
        in("edi") arg5,
        in("ebp") arg6,
        options(nostack),
    );
  }

  ret
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub unsafe fn syscall6(num: isize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize) -> isize {
  let ret: isize;

  unsafe {
    core::arch::asm!(
        "syscall",
        inlateout("rax") num => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        in("r9") arg6,
        clobber_abi("sysv64"),
    );
  }

  ret
}
