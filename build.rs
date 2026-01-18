use std::{
  env,
  path::PathBuf,
};

#[cfg(any(
  target_os = "linux",
  target_os = "windows",
  target_os = "freebsd",
  target_os = "macos"
))]
mod constants {
  pub const OUT_DIR: &str = "OUT_DIR";

  pub const UNABLE_TO_GENERATE: &str = "Unable to generate bindings";
  pub const UNABLE_TO_WRITE: &str = "Unable to write bindings to file";

  #[cfg(target_os = "linux")]
  pub mod linux {
    pub const FUTEX_HEADER: &str = "/usr/include/linux/futex.h";
    pub const FUTEX_TARGET: &str = "linux_futex_bindings.rs";
    pub const UNISTD_TARGET: &str = "linux_unistd_bindings.rs";

    #[cfg(target_arch = "x86_64")]
    pub const UNISTD_HEADER: &str = "/usr/include/asm/unistd_64.h";

    #[cfg(target_arch = "x86")]
    pub const UNISTD_HEADER: &str = "/usr/include/asm/unistd_32.h";
  }
}

#[cfg(any(
  target_os = "linux",
  target_os = "windows",
  target_os = "freebsd",
  target_os = "macos"
))]
mod imports {
  pub use bindgen::Builder;
}

/*

  Generate Linux-specific headers

*/
#[cfg(target_os = "linux")]
fn linux_unistd_headers(builder: imports::Builder, out: &PathBuf) {
  let unistd_binds = builder
    .header(constants::linux::UNISTD_HEADER)
    .generate()
    .expect(constants::UNABLE_TO_GENERATE);

  let out_path = out.join(constants::linux::UNISTD_TARGET);
  unistd_binds
    .write_to_file(&out_path)
    .expect(constants::UNABLE_TO_WRITE);

  println!(
    "cargo:rustc-env=LINUX_UNISTD_BINDINGS={}",
    out_path.display()
  );
}

#[cfg(target_os = "linux")]
fn linux_futex_headers(builder: imports::Builder, out: &PathBuf) {
  let futex_binds = builder
    .header(constants::linux::FUTEX_HEADER)
    .generate()
    .expect(constants::UNABLE_TO_GENERATE);

  let out_path = out.join(constants::linux::FUTEX_TARGET);
  futex_binds
    .write_to_file(&out_path)
    .expect(constants::UNABLE_TO_WRITE);

  println!(
    "cargo:rustc-env=LINUX_FUTEX_BINDINGS={}",
    out_path.display()
  );
}

#[cfg(target_os = "linux")]
fn linux_headers(builder: imports::Builder, out: &PathBuf) {
  linux_unistd_headers(builder.clone(), out);
  linux_futex_headers(builder, out);
}

/*

  Generate Windows-specific headers

*/

#[cfg(target_os = "windows")]
fn windows_headers(_builder: imports::Builder, _out: &PathBuf) {
  todo!("Implement Windows header generation");
}

/*

  Generate FreeBSD-specific headers

*/

#[cfg(target_os = "freebsd")]
fn freebsd_headers(_builder: imports::Builder, _out: &PathBuf) {
  todo!("Implement FreeBSD header generation");
}

/*

  Generate macOS-specific headers

*/

#[cfg(target_os = "macos")]
fn macos_headers(_builder: imports::Builder, _out: &PathBuf) {
  todo!("Implement macOS header generation");
}

fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  #[cfg(any(
    target_os = "linux",
    target_os = "windows",
    target_os = "freebsd",
    target_os = "macos"
  ))]
  {
    let out = PathBuf::from(env::var(constants::OUT_DIR).unwrap());
    let builder = imports::Builder::default().use_core();

    #[cfg(target_os = "linux")]
    linux_headers(builder, &out);

    #[cfg(target_os = "windows")]
    windows_headers(&builder, &out);

    #[cfg(target_os = "freebsd")]
    freebsd_headers(&builder, &out);

    #[cfg(target_os = "macos")]
    macos_headers(&builder, &out);
  }
}
