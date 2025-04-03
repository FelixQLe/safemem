# SafeMem

Below is a polished README for your "SafeMem: C-to-Rust Memory Safety Demo" project, tailored for a GitHub repository. It’s concise, professional, and highlights the project’s purpose, setup, and usage, while leaving room for you to customize details like your GitHub handle or license. This assumes the project structure and goals from our earlier discussion, as of March 28, 2025.

---

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
   git clone https://github.com/[your-username]/safemem.git
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
- Overflow the C buffer: Pass a huge string (e.g., 1000 chars) and watch it break.  
- Use-after-free: Append after freeing in C, then try in Rust (it won’t compile).  
- Debug with `valgrind`: `valgrind --leak-check=full target/debug/safemem`.

## Contributing
Feel free to fork, tweak, or suggest improvements! Open an issue or PR if you’ve got ideas—extra vulnerabilities to demo, performance tweaks, whatever.

## License
[MIT License](LICENSE) - Free to use, modify, and share.

## Acknowledgments
- Inspired by real-world memory safety challenges in OS development.  
- Built with Rust’s fearless concurrency and ownership model.

---

*Made by [FelixQLe-HopLe] to explore C’s chaos and Rust’s calm.*

---
