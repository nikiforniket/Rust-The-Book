# 📚 Rust-The-Book — Code Examples

> Personal solutions & notes while working through [_The Rust Programming Language_ (“TRPL”)](https://doc.rust-lang.org/stable/book/).  
> Every chapter lives in its own directory so you can jump straight to any topic.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
![Rust Version](https://img.shields.io/badge/rustc-1.81%2B-orange)

---

## 📂 Project Layout

```text
.
├── chapter_01/               # “Getting Started”: hello‑world & tooling basics
├── chapter_02/
│   └── guessing_game/        # CLI number‑guessing game
├── chapter_03/               # Common programming concepts (variables, flow…)
├── .gitignore
└── LICENSE                   # GPL‑3.0
```

> **Tip:** use `tree -L 2` to view the repo like above.

---

## 🚀 Quick Start

1. **Install Rust & Cargo**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update stable      # make sure you’re on 1.81 or later
   ```

2. **Clone this repo**

   ```bash
   git clone https://github.com/nikiforniket/Rust-The-Book.git
   cd Rust-The-Book
   ```

3. **Run an example**

   ```bash
   cd chapter_02/guessing_game
   cargo run
   ```

   Building once caches dependencies in `target/`.  
   For an optimized binary:

   ```bash
   cargo build --release
   ```

---

## 🛠️ Chapter Map

| Chapter | Folder | Highlights |
|---------|--------|------------|
| 1 | `chapter_01/` | toolchain install, “Hello, world!” |
| 2 | `chapter_02/guessing_game` | `rand` crate, loops, pattern matching |
| 3 | `chapter_03/` | variables, data types, functions, control flow |

*(More directories will appear as I progress.)*

---

## 🤝 Contributing

Although this is primarily a study repo, contributions are welcome if they:

* Fix typos or clippy warnings.
* Offer alternative solutions (in separate files).
* Improve documentation clarity without altering learning order.

Fork → feature branch → clear commit → PR.

---

## 📜 License

Code is released under **GNU GPL v3**; see [`LICENSE`](./LICENSE) for the full text.

---

## 🙏 Acknowledgements

* [_The Rust Programming Language_](https://doc.rust-lang.org/stable/book/) by Steve Klabnik & Carol Nichols.
* The amazing Rust community for docs, crates, and guidance.