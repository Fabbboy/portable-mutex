# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`portable-mutex` is a `no_std` Rust library implementing a futex-based mutex with minimal overhead. The implementation uses direct syscalls to OS futex primitives, avoiding libc dependencies entirely.

## Build Commands

```bash
# Build the library
cargo build

# Build with release optimizations
cargo build --release

# Build with fallback feature (spinlock instead of futex)
cargo build --features fallback

# Run tests
cargo test

# Format code (uses custom rustfmt.toml)
cargo fmt

# Check code without building
cargo check
```

## Platform Support

**Currently Supported:**
- Linux (x86, x86_64) - fully implemented

**Planned (currently todo!()):**
- Windows
- FreeBSD
- macOS

The crate will fail to compile on unsupported platforms unless the `fallback` feature is enabled.

## Architecture

### Build-Time Code Generation

The build script (`build.rs`) uses `bindgen` to generate Rust bindings from OS headers at compile time:

- **Linux:** Generates bindings for `/usr/include/linux/futex.h` and `/usr/include/asm/unistd_{32,64}.h`
- Generated files are written to `$OUT_DIR` and included via `env!()` macros in `src/futex/bindings.rs`
- Environment variables set: `LINUX_FUTEX_BINDINGS`, `LINUX_UNISTD_BINDINGS`

### Core Components

1. **`src/mutex.rs`** - Public API
   - `Mutex<T>` wrapper around `RawMutex` and `UnsafeCell<T>`
   - Provides `new()` and `lock()` methods
   - `Send` + `Sync` when `T: Send`

2. **`src/raw.rs`** - Low-level mutex implementation
   - Three-state design: `Unlocked`, `Locked`, `Contended`
   - Fast path uses atomic compare-exchange
   - Slow path transitions to `Contended` and calls futex wait/wake
   - Direct futex integration via `src/futex/` module

3. **`src/guard.rs`** - RAII guard
   - `MutexGuard<'m, T>` implements `Deref` and `DerefMut`
   - Unlocks mutex in `Drop` implementation
   - Uses `PhantomData<Cell<()>>` to make guard `!Send`

4. **`src/futex/`** - OS-specific futex syscalls
   - `mod.rs`: Platform dispatch layer
   - `linux.rs`: Linux-specific implementation
   - `linux/x86_64.rs`: Raw syscall wrappers using inline assembly
     - `syscall6()` for x86 (`int 0x80`) and x86_64 (`syscall`)
   - `bindings.rs`: Includes generated constants (`FUTEX_WAIT`, `FUTEX_WAKE`, `__NR_futex`)

### Key Design Patterns

- **No libc dependency:** All syscalls via inline assembly (`core::arch::asm!`)
- **Conditional compilation:** Heavy use of `#[cfg()]` for platform/architecture selection
- **Zero-cost abstraction:** `#[inline(always)]` on critical paths
- **Build-time codegen:** Bindgen generates platform-specific constants

## Code Style

- 2-space indentation (enforced by `rustfmt.toml`)
- 100-character line width
- Vertical imports layout with crate-level granularity
- Requires nightly Rust (see `rust-toolchain.toml`)

## Adding Platform Support

To add a new platform (Windows/FreeBSD/macOS):

1. Update `build.rs` to generate platform-specific bindings
2. Add platform module in `src/futex/`
3. Implement `wait()` and `wake()` syscall wrappers
4. Update dispatch logic in `src/futex/mod.rs`
5. Handle architecture-specific syscall ABIs (like x86 vs x86_64)
