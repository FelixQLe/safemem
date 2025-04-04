# SafeMem: C-to-Rust Memory Safety Demo

**SafeMem** is a small, educational project that demonstrates how Rust eliminates common memory safety vulnerabilities found in C. It features a simple buffer manager implemented in both languages: a vulnerable C version prone to buffer overflows, use-after-free, and double-free errors, and a safe Rust version leveraging ownership and bounds checking. Perfect for learning about memory safety or showcasing Rust’s systems programming strengths!

## Features
- **Vulnerable C Implementation**: A dynamic buffer manager with deliberate memory safety flaws.  
- **Safe Rust Implementation**: A reimagined version using Rust’s `Vec` and ownership model.  
- **Test Harness**: Triggers C vulnerabilities and contrasts them with Rust’s resilience.  
- **OS Relevance**: Mirrors memory management challenges in operating systems.

## Why SafeMem?
Memory safety bugs (e.g., buffer overflows) account for ~70% of serious security vulnerabilities in C/C++ codebases (per Microsoft). SafeMem shows how Rust prevents these at compile time, making it a toy example of why OS developers are eyeing Rust (e.g., Linux kernel, Windows).

## Getting Started

### Prerequisites
- **Rust**: Install via [rustup](https://rustup.rs/) (`cargo` required).  
- **C Compiler**: `gcc` or equivalent for the C component.  
- **Valgrind** (optional): For debugging C memory issues (`sudo apt install valgrind` on Ubuntu).  

### Installation
1. Clone the repo:
   ```bash
   git clone https://github.com/Hop-Le133884/SafeMem_C-to-Rust_Memory_Safety_Demo.git
   cd safemem
   ```
2. Build and run:
   ```bash
   cargo run
   ```

### Usage
- **C Vulnerabilities**: The test harness triggers buffer overflows, use-after-free, and double-free in the C version—expect crashes or undefined behavior.  
- **Rust Safety**: The Rust version runs cleanly, with overflows prevented and deallocation handled automatically.  
- **Output Example**:
  ```
  Testing C vulnerabilities:
  Appended 'Hello'
  Use-after-free triggered
  [Crash or garbage output]

  Testing Rust safety:
  Appended 'Hello': "Hello"
  Buffer dropped safely
  Rust prevents use-after-free
  ```

## Project Structure
```
safemem/
├── c_src/          # Vulnerable C code
│   ├── buffer.c
│   └── buffer.h
├── src/            # Rust code
│   ├── lib.rs      # Safe Buffer implementation
│   └── main.rs     # Test harness with C FFI
├── Cargo.toml      # Rust dependencies
└── build.rs        # C build script
```

## How It Works
- **C Version**:  
  - `buffer_append`: No bounds checking—overflows possible.  
  - `buffer_free`: Frees memory but allows reuse or double-free.  
- **Rust Version**:  
  - `Buffer::append`: Uses `Vec` with bounds checks and overflow protection.  
  - `Drop`: Ensures safe, single deallocation—no manual `free` needed.  
- **FFI**: Rust calls C via `extern "C"` to compare side-by-side.

## Try It Out
Modify `main.rs` to test these specific cases and see C break while Rust holds strong:

- **Buffer Overflows**:  
  1. **Small Overflow**: Append `"Hello"` to a C buffer with `capacity=4`. Exceeds limit, likely corrupts memory. Rust’s `Vec` grows safely.  
     - C: Crashes or prints garbage.  
     - Rust: Succeeds with `"Hello"`.  
  2. **Large Overflow**: Append a 1000-char string (e.g., `"A".repeat(1000)` in Rust, `char big[1000]` in C). C overwrites wildly; Rust handles it.  
     - C: Undefined behavior, possible segfault.  
     - Rust: Allocates and stores cleanly.  

- **Use-After-Free**:  
  1. **Simple Reuse**: Free a C buffer, then append `"World"`. C may corrupt heap; Rust won’t compile.  
     - C: Garbage output or crash.  
     - Rust: `r_buf.append("World")` after `drop` fails at compile time.  
  2. **Double Append**: Free a C buffer, append `"Foo"`, then `"Bar"`. C’s chaos escalates; Rust stays safe.  
     - C: Increasingly unpredictable behavior.  
     - Rust: Blocked by ownership rules.  

- **Double-Free**:  
  1. **Basic Double-Free**: Call `buffer_free` twice on the same C buffer. Often crashes (e.g., heap corruption). Rust’s `Drop` runs once.  
     - C: Segfault or silent error.  
     - Rust: No equivalent—memory freed automatically.  
  2. **Free-and-Reuse**: Free a C buffer, append `"Test"`, free again. C doubles down on errors; Rust avoids the issue.  
     - C: Likely aborts (e.g., `glibc` detects double-free).  
     - Rust: `Drop` ensures single cleanup.  

- **Debugging**: Run C tests with `valgrind --leak-check=full target/debug/safemem` to see overflows and leaks in action.

## Contributing
Feel free to fork, tweak, or suggest improvements! Open an issue or PR if you’ve got ideas—extra vulnerabilities to demo, performance tweaks, whatever.

## Acknowledgments
- Inspired by real-world memory safety challenges in OS development.  
- Built with Rust’s fearless concurrency and ownership model.

---

*Made by [HopLe133884] to explore C’s chaos and Rust’s calm.*

---
