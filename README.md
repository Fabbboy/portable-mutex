# portable-mutex

A `no_std` futex-based mutex implementation with minimal overhead.

## Work in Progress

This project is currently **under active development**. Despite its name suggesting portability, `portable-mutex` is **not yet portable** across multiple platforms. Currently, only Linux (x86/x86_64) is fully implemented.

### Platform Support Status

| Platform | x86 | x86_64 | ARM | ARM64 | RV32 | RV64 |
| -------- | --- | ------ | --- | ----- | ---- | ---- |
| Linux    | ✓   | ✓      | ✓   | ✓     | ✗    | ✗    |
| Windows  | ✗   | ✗      | —   | ✗     | —    | —    |
| FreeBSD  | ✗   | ✗      | ✗   | ✗     | ✗    | ✗    |
| macOS    | —   | —      | —   | ✗     | —    | —    |

Backend implementations for Windows, FreeBSD, and macOS are planned but not yet written. Attempting to build on unsupported platforms will result in a compile-time error.

## Features

- `no_std` compatible
- Direct syscall implementation (no libc dependency)
- Three-state mutex design for efficient contention handling

## Usage

```rust
use portable_mutex::prelude::*;

let mutex = Mutex::new(0);
let mut guard = mutex.lock();
*guard += 1;
```
