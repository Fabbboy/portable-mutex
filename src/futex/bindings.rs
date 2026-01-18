#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[cfg(target_os = "linux")]
include!(env!("LINUX_FUTEX_BINDINGS"));

#[cfg(target_os = "linux")]
include!(env!("LINUX_UNISTD_BINDINGS"));
