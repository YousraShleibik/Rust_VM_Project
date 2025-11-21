# Rust Lox VM & Compiler

This project is a small **bytecode virtual machine** and **compiler** for a subset of the Lox language, implemented in Rust.  

It is roughly based on the book _Crafting Interpreters_ by Robert Nystrom, but adapted to Rust and to the COMP 4701 weekly assignments.

---
rust_vm_project/
├── src/
│   ├── compiler.rs     # Pratt parser and bytecode compiler
│   ├── chunk.rs        # Chunk of bytecode instructions and constants
│   ├── scanner.rs      # Lexical scanner for tokenizing Lox source
│   ├── token.rs        # Token and TokenType definitions
│   ├── value.rs        # Lox Value representation (numbers, strings, bools)
│   ├── vm.rs           # Virtual Machine implementation
│   └── lib.rs          # Library entrypoint and internal unit tests
├── tests/
│   └── expr_eval.rs    # Integration tests for arithmetic and expressions
├── Cargo.toml          # Rust project configuration
└── README.md           # You are here 🚀

## ✨ Features

As of Weekly Assignments 1–6, the project supports:

### Core VM & Types
- Stack-based **virtual machine** with a `Chunk` of bytecode and constants.
- `Number` type alias:  
  ```rust
  pub type Number = i16;

- Value enum representing Lox values:

pub enum Value {
    ValBool(bool),
    ValNumber(Number),
    ValNil,
    ValString(String),
}



- Arithmetic operations on numbers:

+, -, *, /, %

- Comparison and equality operators:

==, !=, <, <=, >, >=

- Truthiness rules and logical NOT:

!expr

nil and false are falsey, everything else is truthy.

### Bytecode / OpCodes

The VM executes a set of opcodes, including:

Core:

OpConstant

OpReturn

OpNegate

OpAdd, OpSubtract, OpMultiply, OpDivide, OpModulo

Values / logic:

OpNil, OpTrue, OpFalse

OpNot

OpEqual, OpGreater, OpLess

Statements:

OpPrint – print top of stack

OpPop – discard top of stack (used for expression statements)

Globals:

OpDefineGlobal – define a global variable

OpGetGlobal – read a global variable

### 🧠 Compiler & Parser

The compiler lives in src/compiler.rs and uses a Pratt parser with a precedence table to parse expressions.

### Key components

Scanner (from scanner.rs): converts source text → Tokens.

Parser:

Tracks current and previous tokens.

Uses a Pratt table (HashMap<TokenType, ParseRule>) to define prefix/infix behavior and precedences.

Compiler:

Compiles expressions and statements into a Chunk of bytecode.

Emits opcodes via helper methods emit_byte, emit_bytes, emit_constant, etc.

Supported syntax

Expressions:

1 + 2 * 3
(10 - 3) / 2
-5
!false
1 < 2 == true


Literals:

123
true
false
nil


Statements:

print 1 + 2;
1 + 2 + 3;          // expression statement, result is discarded with OP_POP

var x = 10;
var y;
print x + 5;


### Global variables:

Declared with var:

var x = 10;

var y;

Used in expressions:

print x + 5;

### Implemented via:

OpDefineGlobal storing into VirtualMachine.globals: HashMap<String, Value>

### OpGetGlobal loading from that map

🖥 Virtual Machine

The VM is defined in src/lib.rs as VirtualMachine:

pub struct VirtualMachine {
    pub chunk: Option<Chunk>,
    pub ip: usize,                  // instruction pointer
    pub stack: Vec<Value>,          // value stack
    pub globals: HashMap<String, Value>, // global variables
}


### Important methods:

VirtualMachine::init_machine() – construct an empty VM.

interpret(source_code: &str) – compile and execute a Lox source string.

interpret_chunk(chunk: Chunk) – execute an already compiled chunk.

run() – main bytecode execution loop.

runtime_error(&mut self, message: &str) – runtime error reporting.

### Helpers:

push, pop

binary_op_number, binary_op_cmp

is_falsey, values_equal, as_number

The VM also has a print_value helper to print Values:

nil

true / false

numeric values

strings

### 🧪 Testing

There are two layers of tests:

1. Unit tests in src/lib.rs

These test:

Opcode ↔ byte mappings (opcode_to_u8, u8_to_opcode)

Chunk behavior (constants, lines, bytecode layout)

Disassembly offsets and unknown opcodes

VM arithmetic and error handling:

simple arithmetic program

division by zero → runtime error

stack underflow → runtime error

Part 6 behavior:

print statements

expression statements using OpPop

globals (OpDefineGlobal, OpGetGlobal)

undefined global → runtime error

Run them with:

cargo test --lib

2. Integration tests in tests/expr_eval.rs

These treat small strings like "1+8", "2+3*4", etc., as whole programs, and check:

Correct arithmetic results

Operator precedence and associativity

Unary minus and nested parentheses

Division behavior & division by zero runtime error

Run them with:

cargo test --test expr_eval -- --nocapture

🔧 Building & Running
Build the library
cargo build

Run all tests
cargo test

Running the interpreter

If your main.rs is wired to call into VirtualMachine and compile a file:

cargo run -- examples.lox


(Replace examples.lox with any Lox source file you want to run.)

📚 Internals / Design Notes

Expressions are parsed with a Pratt parser using a ParseRule table keyed by TokenType.

Operator precedence is encoded via the Precedence enum.

Each top-level Lox program is compiled either as:

a sequence of declarations/statements, or

a single expression (for the expression test suite).

The VM follows a classic stack machine design:

constants loaded via OpConstant

operations consume values from the stack and push results back.

