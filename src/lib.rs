pub mod scanner;
pub mod compiler;
use crate::compiler::Compiler;
pub use scanner::{Scanner, Token, TokenType};
pub type Number = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    ValBool(bool),
    ValNumber(Number),
    ValNil,
}
//operation codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    OpReturn, 
    OpConstant, 
    OpNegate, 
    OpAdd, 
    OpSubtract, 
    OpMultiply, 
    OpDivide,
    OpModulo,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,

}

//helper function to convert OpCode to u8
pub fn opcode_to_u8(op: OpCode) -> u8 {
    match op {
        OpCode::OpReturn   => 0x00,
        OpCode::OpConstant => 0x01,
        OpCode::OpNegate   => 0x02,
        OpCode::OpAdd      => 0x03,
        OpCode::OpSubtract => 0x04,
        OpCode::OpMultiply => 0x05,
        OpCode::OpDivide   => 0x06,
        OpCode::OpModulo   => 0x07,
        OpCode::OpNil       => 0x08,
        OpCode::OpTrue      => 0x09,
        OpCode::OpFalse     => 0x0A,
        OpCode::OpNot       => 0x0B,
        OpCode::OpEqual     => 0x0C,
        OpCode::OpGreater   => 0x0D,
        OpCode::OpLess      => 0x0E,

    }
}

//helper function to convert u8 to OpCode
pub fn u8_to_opcode(b: u8) -> Option<OpCode> {
    Some(match b {
        0x00 => OpCode::OpReturn,
        0x01 => OpCode::OpConstant,
        0x02 => OpCode::OpNegate,
        0x03 => OpCode::OpAdd,
        0x04 => OpCode::OpSubtract,
        0x05 => OpCode::OpMultiply,
        0x06 => OpCode::OpDivide,
        0x07 => OpCode::OpModulo,
        0x08 => OpCode::OpNil,
        0x09 => OpCode::OpTrue,
        0x0A => OpCode::OpFalse,
        0x0B => OpCode::OpNot,
        0x0C => OpCode::OpEqual,
        0x0D => OpCode::OpGreater,
        0x0E => OpCode::OpLess,
        _ => return None,
    })
}

#[derive(Debug)]
pub struct Chunk {
    //constants: Vec<Value>, //constants used in the bytecode
    code: Vec<u8>,       //bytecode
    lines: Vec<u8>,  //line numbers for each bytecode instruction
    values: Vec<Value>, //constants used in the bytecode
}

impl Chunk {
    fn fmt_value(v: &Value) -> String {
    match v {
        Value::ValBool(b)   => format!("{}", b),
        Value::ValNumber(n) => format!("{}", n),
        Value::ValNil       => "nil".to_string(),
    }
    }
    pub fn init_chunk() -> Self {
        Chunk {
            //constants: Vec::new(),
            code: Vec::new(),
            lines: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn write_to_chunk(&mut self, byte: u8, line: u8) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.values.push(value);
        (self.values.len() - 1) as u8 //return the index of the added constant
        
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0usize;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        use std::fmt::Write as _;
        let line = self.lines.get(offset).copied().unwrap_or(0);
        let mut out = String::new();

        let byte = self.code[offset];
        let next = if let Some(op) = u8_to_opcode(byte) {
            match op {
                OpCode::OpReturn => {
                    let _ = write!(out, "{offset:04}  line {:>4}  {:<12}", line, "OpReturn");
                    offset + 1
                }
                OpCode::OpConstant => {
                    let idx = self.code.get(offset + 1).copied().unwrap_or(0);
                    let shown = self
                        .values
                        .get(idx as usize)
                        .map(Chunk::fmt_value)
                        .unwrap_or_else(|| "?".into());
                    let _ = write!(
                        out,
                        "{offset:04}  line {:>4}  {:<12} idx={:<3} value={}",
                        line, "OpConstant", idx, shown
                    );
                    offset + 2
                }
                OpCode::OpNegate => {
                    let _ = write!(out, "{offset:04}  line {:>4}  {:<12}", line, "OpNegate");
                    offset + 1
                }
                OpCode::OpAdd => {
                    let _ = write!(out, "{offset:04}  line {:>4}  {:<12}", line, "OpAdd");
                    offset + 1
                }
                OpCode::OpSubtract => {
                    let _ = write!(out, "{offset:04}  line {:>4}  {:<12}", line, "OpSubtract");
                    offset + 1
                }
                OpCode::OpMultiply => {
                    let _ = write!(out, "{offset:04}  line {:>4}  {:<12}", line, "OpMultiply");
                    offset + 1
                }
                OpCode::OpDivide => {
                    let _ = write!(out, "{offset:04}  line {:>4}  {:<12}", line, "OpDivide");
                    offset + 1
                }
                OpCode::OpModulo => {
                    let _ = write!(out, "{offset:04}  line {:>4}  {:<12}", line, "OpModulo");
                offset + 1
                }
                OpCode::OpNil     => {
                     let _ = write!(out, "{offset:04}  line {:>4}  OpNil",   line); 
                offset + 1 
                }
                OpCode::OpTrue    => {
                     let _ = write!(out, "{offset:04}  line {:>4}  OpTrue",  line); 
                offset + 1 
                }
                OpCode::OpFalse   => {
                     let _ = write!(out, "{offset:04}  line {:>4}  OpFalse", line); 
                offset + 1 
                }
                OpCode::OpNot     => {
                     let _ = write!(out, "{offset:04}  line {:>4}  OpNot",   line); 
                offset + 1 
                }
                OpCode::OpEqual   => {
                     let _ = write!(out, "{offset:04}  line {:>4}  OpEqual", line); 
                offset + 1 
                }
                OpCode::OpGreater => {
                     let _ = write!(out, "{offset:04}  line {:>4}  OpGreater", line); 
                offset + 1 
                }
                OpCode::OpLess    => {
                     let _ = write!(out, "{offset:04}  line {:>4}  OpLess",  line); 
                offset + 1 
                }
            }
        } else {
            let _ = write!(out, "{offset:04}  line {:>4}  {:<12} 0x{:02X} (unknown)", line, "????", byte);
            offset + 1
        };

        println!("{out}");
        next
    }
}

pub struct VirtualMachine {
    pub chunk: Option<Chunk>,
    pub ip: usize,
    pub stack: Vec<Value>,
}

impl VirtualMachine {
    pub fn init_machine() -> Self {
        VirtualMachine {
            chunk: None,
            ip: 0,
            stack: Vec::new(),
        }
    }

pub fn interpret(&mut self, source_code: &str) -> InterpretResult {
    let mut the_compiler: Compiler = Compiler::init_compiler();
    if !the_compiler.compile(source_code) {
        println!("Finished Compiling");
        return InterpretResult::InterpretCompileError;
    }
    println!("Starting run");
    self.chunk = Some(the_compiler.get_chunk());
    self.run()
}

pub fn interpret_chunk(&mut self, chunk: Chunk) -> InterpretResult {
    self.chunk = Some(chunk);
    self.ip = 0;
    self.run()
}

    pub fn run(&mut self) -> InterpretResult {
    loop {
        let chunk = match &self.chunk {
            Some(c) => c,
            None => return InterpretResult::InterpretRuntimeError,
        };

        if self.ip >= chunk.code.len() {
            return InterpretResult::InterpretRuntimeError;
        }

        let instruction = chunk.code[self.ip];
        self.ip += 1;

        match u8_to_opcode(instruction) {
            Some(OpCode::OpReturn) => {
                // Stop execution
                return InterpretResult::InterpretSuccess;
            }
            Some(OpCode::OpConstant) => {
                let idx = chunk.code.get(self.ip).copied().unwrap_or(0) as usize;
                self.ip += 1;
                if let Some(v) = chunk.values.get(idx).copied() {
                    self.push(v);
                } else {
                    self.runtime_error("Bad constant index.");
                    return InterpretResult::InterpretRuntimeError;
                }
            }
            Some(OpCode::OpNegate) => {
                let v = match self.pop() {
                    Some(v) => v,
                    None => { self.runtime_error("Stack underflow."); return InterpretResult::InterpretRuntimeError; }
                };
                if let Some(n) = Self::as_number(&v) {
                    self.push(Value::ValNumber(-n));
                } else {
                    self.runtime_error("Operand must be a number.");
                    return InterpretResult::InterpretRuntimeError;
                }
            }
            Some(OpCode::OpAdd)      => { if self.binary_op_number(|a,b| a + b).is_err() { return InterpretResult::InterpretRuntimeError; } }
            Some(OpCode::OpSubtract) => { if self.binary_op_number(|a,b| a - b).is_err() { return InterpretResult::InterpretRuntimeError; } }
            Some(OpCode::OpMultiply) => { if self.binary_op_number(|a,b| a * b).is_err() { return InterpretResult::InterpretRuntimeError; } }
            Some(OpCode::OpDivide)   => {
                let (b, a) = match (self.pop(), self.pop()) {
                    (Some(x), Some(y)) => (x, y),
                    _ => { self.runtime_error("Stack underflow."); return InterpretResult::InterpretRuntimeError; }
                };
                let (Some(bn), Some(an)) = (Self::as_number(&b), Self::as_number(&a)) else {
                    self.runtime_error("Operands must be numbers."); return InterpretResult::InterpretRuntimeError;
                };
                if bn == 0 {
                    self.runtime_error("Division by zero.");
                    return InterpretResult::InterpretRuntimeError;
                }
                self.push(Value::ValNumber(an / bn));
            }
            Some(OpCode::OpModulo)   => {
                let (b, a) = match (self.pop(), self.pop()) {
                    (Some(x), Some(y)) => (x, y),
                    _ => { self.runtime_error("Stack underflow."); return InterpretResult::InterpretRuntimeError; }
                };
                let (Some(bn), Some(an)) = (Self::as_number(&b), Self::as_number(&a)) else {
                    self.runtime_error("Operands must be numbers."); return InterpretResult::InterpretRuntimeError;
                };
                if bn == 0 {
                    self.runtime_error("Modulo by zero.");
                    return InterpretResult::InterpretRuntimeError;
                }
                self.push(Value::ValNumber(an % bn));
            }
            Some(OpCode::OpNil)   => self.push(Value::ValNil),
            Some(OpCode::OpTrue)  => self.push(Value::ValBool(true)),
            Some(OpCode::OpFalse) => self.push(Value::ValBool(false)),
            Some(OpCode::OpNot) => {
                let v = match self.pop() {
                    Some(v) => v,
                    None => { self.runtime_error("Stack underflow."); return InterpretResult::InterpretRuntimeError; }
                };
                self.push(Value::ValBool(Self::is_falsey(&v)));
            }
            Some(OpCode::OpEqual)   => {
                let (b, a) = match (self.pop(), self.pop()) {
                    (Some(x), Some(y)) => (x, y),
                    _ => { self.runtime_error("Stack underflow."); return InterpretResult::InterpretRuntimeError; }
                };
                self.push(Value::ValBool(Self::values_equal(&a, &b)));
            }
            Some(OpCode::OpGreater) => {
                if self.binary_op_cmp(|a,b| a > b).is_err() { return InterpretResult::InterpretRuntimeError; }
            }
            Some(OpCode::OpLess)    => {
                if self.binary_op_cmp(|a,b| a < b).is_err() { return InterpretResult::InterpretRuntimeError; }
            }
            None => return InterpretResult::InterpretRuntimeError,
        }
    }
} 

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    fn runtime_error(&mut self, message: &str) {
        println!("{}", message);
        if let Some(chunk) = &self.chunk {
            let ip = self.ip.saturating_sub(1);
            let line = *chunk.lines.get(ip).unwrap_or(&0);
            println!("[line {}] in script", line);
        }
    }

    #[inline]
    fn is_falsey(v: &Value) -> bool {
        matches!(v, Value::ValNil | Value::ValBool(false))
    }

    #[inline]
    fn values_equal(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::ValNil, Value::ValNil) => true,
            (Value::ValBool(x),   Value::ValBool(y))   => x == y,
            (Value::ValNumber(x), Value::ValNumber(y)) => x == y,
            _ => false,
        }
    }

    #[inline]
    fn as_number(v: &Value) -> Option<Number> {
        if let Value::ValNumber(n) = *v { Some(n) } else { None }
    }

    fn binary_op_number<F>(&mut self, f: F) -> Result<(), ()>
    where
        F: Fn(Number, Number) -> Number,
    {
        let (b, a) = match (self.pop(), self.pop()) {
            (Some(x), Some(y)) => (x, y),
            _ => { self.runtime_error("Stack underflow."); return Err(()); }
        };
        let (Some(bn), Some(an)) = (Self::as_number(&b), Self::as_number(&a)) else {
            self.runtime_error("Operands must be numbers.");
            return Err(());
        };
        self.push(Value::ValNumber(f(an, bn)));
        Ok(())
    }

    fn binary_op_cmp<F>(&mut self, f: F) -> Result<(), ()>
    where
        F: Fn(Number, Number) -> bool,
    {
        let (b, a) = match (self.pop(), self.pop()) {
            (Some(x), Some(y)) => (x, y),
            _ => { self.runtime_error("Stack underflow."); return Err(()); }
        };
        let (Some(bn), Some(an)) = (Self::as_number(&b), Self::as_number(&a)) else {
            self.runtime_error("Operands must be numbers.");
            return Err(());
        };
        self.push(Value::ValBool(f(an, bn)));
        Ok(())
    }

    pub fn interpret_source(&mut self, source_code: &str) -> InterpretResult { 
    self.compile(source_code);                                             
    InterpretResult::InterpretSuccess                                      
    }
    
    pub fn compile(&mut self, source_code: &str) {         
    use crate::{Scanner, TokenType};                   

    let mut scanner = Scanner::init_scanner(source_code); 
    let mut line: usize = 0;                            

    loop {                                              
        let token = scanner.scan_token();               

        if token.line != line {                         
            print!("{:4} ", token.line);                
            line = token.line;                          
        } else {                                        
            print!("   | ");                            
        }                                               

        let text = String::from_utf8(token.value.clone()).ok(); 
        let len = token.value.len();                             
        println!("{:?} {}, {:?}", token.token_type, len, text);  

        match token.token_type {                      
            TokenType::TokenEof => break,             
            _ => {}                                   
        }                                             
    }                                                 
    } 
        
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    InterpretSuccess,
    InterpretCompileError,
    InterpretRuntimeError,
}


    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_mappings_roundtrip() {
        let table = [
            (OpCode::OpReturn,   0x00),
            (OpCode::OpConstant, 0x01),
            (OpCode::OpNegate,   0x02),
            (OpCode::OpAdd,      0x03),
            (OpCode::OpSubtract, 0x04),
            (OpCode::OpMultiply, 0x05),
            (OpCode::OpDivide,   0x06),
            (OpCode::OpModulo,   0x07),
        ];

        for (op, byte) in table {
            assert_eq!(opcode_to_u8(op), byte, "opcode_to_u8 mismatch for {op:?}");
            assert_eq!(u8_to_opcode(byte), Some(op), "u8_to_opcode mismatch for 0x{byte:02X}");
        }

        // Unknown opcode should map to None
        assert_eq!(u8_to_opcode(0xFF), None);
    }

    #[test]
fn chunk_write_and_constants_basics() {
    let mut c = Chunk::init_chunk();

    // Add a couple constants; verify indices and stored values.
    let i0 = c.add_constant(Value::ValNumber(15));
    let i1 = c.add_constant(Value::ValNumber(42));
    assert_eq!(i0, 0);
    assert_eq!(i1, 1);
    assert_eq!(c.values[i0 as usize], Value::ValNumber(15));
    assert_eq!(c.values[i1 as usize], Value::ValNumber(42)); // <-- was i0,42 before

    // Write opcode + operand pairs, then a Return.
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 10);
    c.write_to_chunk(i0, 10);

    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 11);
    c.write_to_chunk(i1, 11);

    c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 12);

    assert_eq!(c.code.len(), c.lines.len());
    assert_eq!(c.lines, vec![10, 10, 11, 11, 12]);

    assert_eq!(u8_to_opcode(c.code[0]), Some(OpCode::OpConstant));
    assert_eq!(c.code[1], i0);
    assert_eq!(u8_to_opcode(c.code[2]), Some(OpCode::OpConstant));
    assert_eq!(c.code[3], i1);
    assert_eq!(u8_to_opcode(c.code[4]), Some(OpCode::OpReturn));
}


    #[test]
fn disassemble_instruction_offsets_and_unknown() {
    let mut c = Chunk::init_chunk();

    // Build: OpConstant idx0 | OpConstant idx1 | OpAdd | 0xFF(unknown) | OpReturn
    let i0 = c.add_constant(Value::ValNumber(10));
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 1);
    c.write_to_chunk(i0, 1);

    let i1 = c.add_constant(Value::ValNumber(20));
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 2);
    c.write_to_chunk(i1, 2);

    c.write_to_chunk(opcode_to_u8(OpCode::OpAdd), 3);

    // Unknown opcode (should still advance by 1)
    c.write_to_chunk(0xFF, 4);

    c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 5);

    let mut off = 0usize;
    off = c.disassemble_instruction(off); assert_eq!(off, 2);
    off = c.disassemble_instruction(off); assert_eq!(off, 4);
    off = c.disassemble_instruction(off); assert_eq!(off, 5);
    off = c.disassemble_instruction(off); assert_eq!(off, 6);
    off = c.disassemble_instruction(off); assert_eq!(off, 7);

    assert_eq!(c.code.len(), c.lines.len());
    assert_eq!(c.lines, vec![1, 1, 2, 2, 3, 4, 5]);
}

    #[test]
fn vm_exec_simple_arith() {
    // ((((8 + 2) - 3) * 4) / 5) % 3 = 2; negate -> -2
    let mut c = Chunk::init_chunk();
    let l = 1;

    let i8 = c.add_constant(Value::ValNumber(8));
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(i8, l);

    let i2 = c.add_constant(Value::ValNumber(2));
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(i2, l);

    c.write_to_chunk(opcode_to_u8(OpCode::OpAdd), l);

    let i3 = c.add_constant(Value::ValNumber(3));
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(i3, l);

    c.write_to_chunk(opcode_to_u8(OpCode::OpSubtract), l);

    let i4 = c.add_constant(Value::ValNumber(4));
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(i4, l);

    c.write_to_chunk(opcode_to_u8(OpCode::OpMultiply), l);

    let i5 = c.add_constant(Value::ValNumber(5));
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(i5, l);

    c.write_to_chunk(opcode_to_u8(OpCode::OpDivide), l);

    let imod = c.add_constant(Value::ValNumber(3));
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(imod, l);

    c.write_to_chunk(opcode_to_u8(OpCode::OpModulo), l);
    c.write_to_chunk(opcode_to_u8(OpCode::OpNegate), l);
    c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

    let mut vm = VirtualMachine::init_machine();
    let res = vm.interpret_chunk(c);
    assert_eq!(res, InterpretResult::InterpretSuccess);
    assert_eq!(vm.stack.last().copied(), Some(Value::ValNumber(-2)));
}


#[test]
fn vm_divide_by_zero_runtime_error() {
    let mut c = Chunk::init_chunk();
    let l = 1;
    let a = c.add_constant(Value::ValNumber(10));
    let b = c.add_constant(Value::ValNumber(0));

    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(a, l);
    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(b, l);
    c.write_to_chunk(opcode_to_u8(OpCode::OpDivide), l);
    c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

    let mut vm = VirtualMachine::init_machine();
    let res = vm.interpret_chunk(c);
    assert_eq!(res, InterpretResult::InterpretRuntimeError);
}


#[test]
fn vm_stack_underflow_runtime_error() {
    // Attempt to add with only one value on the stack.
    let mut c = Chunk::init_chunk();
    let l = 1;
    let a = c.add_constant(Value::ValNumber(5));

    c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(a, l);
    c.write_to_chunk(opcode_to_u8(OpCode::OpAdd), l);  // needs two values
    c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

    let mut vm = VirtualMachine::init_machine();
    let res = vm.interpret_chunk(c);
    assert_eq!(res, InterpretResult::InterpretRuntimeError);
}

}
