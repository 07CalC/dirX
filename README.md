# 📁 vtreex

A fast and colorful Rust CLI tool to print directory trees — like `tree`, but better.  
Includes stats, ignore filters, text export, and human-readable performance timing.

![2025-06-20-134204_hyprshot](https://github.com/user-attachments/assets/cd591ad3-02b5-4843-b762-07130fbf69cc)

---

## 🚀 Features

- ✅ Beautiful Unicode tree output with colors
- 📦 Built-in ignore list (`.git`, `node_modules`, etc.)
- 🧹 `--ignore` and `--include` for custom control
- 📊 Optional stats: number of files, dirs, and total size
- ⏱️ Human-readable processing time
- 📄 Save output to a `.txt` file

---

## 📦 Installation

### Prerequisites 
- `rust` and `cargo`
👉 Install via rustup.rs:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then restart your terminal and verify:
```bash
rustc --version
cargo --version
```

## Install from crates.io
```bash
cargo install vtreex
```

### Build from source

```bash
git clone https://github.com/07calc/vtreex.git
cd vtreex
cargo build --release
cargo install --path .
```

## 📂 Examples
### Basic usage
```bash
vtreex ${path} ( . by default)
```
### Limit depth
```bash
vtreex -d 3
```
### show stats
```bash
vtreex --stats
```
### Output to a file
```bash
vtreex --output tree.txt
```
Include ignored folders (e.g., .git)
```bash
vtreex --include .git
```

### 📊 Sample Output
```bash
.
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs

📁 1 directories, 📄 4 files
📦 Total size: 12.87 kB

⏱️ Processed in 124μs
```


