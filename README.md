
# 🦀 Rust Lox VM & Compiler

A lightweight bytecode virtual machine and compiler written in Rust — inspired by Robert Nystrom’s **Crafting Interpreters**.
This project implements the backend of the Lox language, including:

* **Scanner (tokenizer)**
* **Pratt parser**
* **Bytecode compiler**
* **Stack-based virtual machine (VM)**

---

## 🚀 Features

* Arithmetic expressions: `+`, `-`, `*`, `/`, `%`
* Unary operators: `-`, `!`
* Boolean literals: `true`, `false`
* `nil` support
* Grouping with parentheses
* **Global variables** (`var x = 10;`)
* Variable lookup and usage
* `print` statements
* Expression statements (`1+2;`)
* Error handling & synchronization
* Dual compile paths:

  * Normal: statements (`print 3;`)
  * Expression-only mode: `vm.interpret("1+2")`

---

## 📁 Project Structure

```
rust_vm_project/
├── src/
│   ├── compiler.rs         # Parser + Compiler
│   ├── scanner.rs          # Lexer / Tokenizer
│   ├── chunk.rs            # Bytecode & constants
│   ├── token.rs            # Token types
│   ├── value.rs            # Lox Values
│   ├── vm.rs               # Virtual Machine
│   └── lib.rs              # Library root + unit tests
├── tests/
│   └── expr_eval.rs        # Integration tests
├── Cargo.toml
└── README.md
```

---

## 🧠 How It Works

### 1. **Scanner**

Converts raw source into a list of `Token`s.

### 2. **Compiler**

Uses a **Pratt parser** to handle:

* operator precedence
* unary/binary operations
* grouping
* literal values
* variable declarations
* print statements

Produces a `Chunk` containing:

* bytecode instructions
* constant pool

### 3. **Virtual Machine**

Executes bytecode using a stack machine model:

* `OpConstant`
* `OpAdd`, `OpSubtract`, `OpMultiply`, `OpDivide`
* `OpNegate`, `OpNot`
* `OpEqual`, `OpGreater`, `OpLess`
* `OpDefineGlobal`, `OpGetGlobal`
* `OpPrint`
* `OpPop`
* `OpReturn`

---

## ⚙️ Building & Running

### **Run all tests**

```bash
cargo test
```

### **Run library (unit) tests**

```bash
cargo test --lib
```

### **Run expression evaluation tests**

```bash
cargo test --test expr_eval -- --nocapture
```

---

## 🧪 Example

Input:

```lox
print 1 + 2 * (3 - 1);
```

Output:

```
5
```

Expression-only mode:

```rust
vm.interpret("1+2");
```

Leaves `3` at the top of the VM stack.

---

## 🧵 Development Notes

* The project supports both **statement mode** and **expression-only mode**.
* `interpret_source()` delegates to a smart mode that auto-detects expressions without semicolons.
* Use disassembly for debugging:

```rust
chunk.disassemble("my_chunk");
```

---

## 📜 License

MIT © 2025
This project is for educational purposes — feel free to use and extend.

