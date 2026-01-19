use std::env;

#[cfg(not(target_os = "windows"))]
mod generator {
  use std::{
    env,
    path::PathBuf,
  };

  use bindgen::Builder;

  const OUT_DIR: &str = "OUT_DIR";
  const UNABLE_TO_GENERATE: &str = "Unable to generate bindings";
  const UNABLE_TO_WRITE: &str = "Unable to write bindings to file";

  #[cfg(target_os = "linux")]
  mod linux {
    pub const FUTEX_HEADER: &str = "/usr/include/linux/futex.h";
    pub const UNISTD_HEADER: &str = "/usr/include/asm/unistd.h";
    pub const FUTEX_TARGET: &str = "linux_futex_bindings.rs";
    pub const UNISTD_TARGET: &str = "linux_unistd_bindings.rs";
  }

  /*

    Generate Linux-specific headers

  */
  #[cfg(target_os = "linux")]
  pub fn linux_headers(out: &PathBuf) {
    let builder = Builder::default().use_core();

    let unistd_binds = builder
      .clone()
      .header(linux::UNISTD_HEADER)
      .generate()
      .expect(UNABLE_TO_GENERATE);

    let out_path = out.join(linux::UNISTD_TARGET);
    unistd_binds
      .write_to_file(&out_path)
      .expect(UNABLE_TO_WRITE);

    println!(
      "cargo:rustc-env=LINUX_UNISTD_BINDINGS={}",
      out_path.display()
    );

    let futex_binds = builder
      .header(linux::FUTEX_HEADER)
      .generate()
      .expect(UNABLE_TO_GENERATE);

    let out_path = out.join(linux::FUTEX_TARGET);
    futex_binds.write_to_file(&out_path).expect(UNABLE_TO_WRITE);

    println!(
      "cargo:rustc-env=LINUX_FUTEX_BINDINGS={}",
      out_path.display()
    );
  }

  /*

    Generate FreeBSD-specific headers

  */
  #[cfg(target_os = "freebsd")]
  pub fn freebsd_headers(_out: &PathBuf) {
    todo!("Implement FreeBSD header generation");
  }

  /*

    Generate macOS-specific headers

  */
  #[cfg(target_os = "macos")]
  pub fn macos_headers(_out: &PathBuf) {
    todo!("Implement macOS header generation");
  }

  pub fn get_out_dir() -> PathBuf {
    PathBuf::from(env::var(OUT_DIR).unwrap())
  }
}

fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

  match target_os.as_str() {
    "linux" => {
      #[cfg(target_os = "linux")]
      generator::linux_headers(&generator::get_out_dir());
    }
    "freebsd" => {
      #[cfg(target_os = "freebsd")]
      generator::freebsd_headers(&generator::get_out_dir());
    }
    "macos" => {
      #[cfg(target_os = "macos")]
      generator::macos_headers(&generator::get_out_dir());
    }
    _ => {}
  }
}
