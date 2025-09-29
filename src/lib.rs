pub type Value = u8;

//operation codes
pub enum OpCode {
    OpReturn, 
    OpConstant, 
    OpNegate, 
    OpAdd, 
    OpSubtract, 
    OpMultiply, 
    OpDivide,

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
        _ => return None,
    })
}

pub struct Chunk {
    //constants: Vec<Value>, //constants used in the bytecode
    code: Vec<u8>,       //bytecode
    lines: Vec<u8>,  //line numbers for each bytecode instruction
    values: Vec<Value>, //constants used in the bytecode
}

impl Chunk {
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
                    // format: [OpConstant][const_index]
                    let idx = self.code.get(offset + 1).copied().unwrap_or(0);
                    let value = self.values.get(idx as usize).copied();
                    let _ = write!(
                        out,
                        "{offset:04}  line {:>4}  {:<12} idx={:<3} value={:?}",
                        line, "OpConstant", idx, value
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
            }
        } else {
            let _ = write!(out, "{offset:04}  line {:>4}  {:<12} 0x{:02X} (unknown)", line, "????", byte);
            offset + 1
        };

        println!("{out}");
        next
    }
}

    //pub fn disassemble(&self) {
       // println!("== CHUNK DUMP ==");
        //println!("code     : {:?}", self.code);
        //println!("lines    : {:?}", self.lines);
        //println!("constants: {:?}", self.values);
    //}

    //pub fn disassemble_instruction(&self, offset: usize) -> usize {
      //  print!("{:04} ", offset); // show instruction index
      //  let instruction = self.code[offset];

        // match u8_to_opcode(instruction) {
        //    Some(OpCode::OpReturn) => {
        //        println!("OP_RETURN");
        //        offset + 1
        //    }
        //    Some(OpCode::OpConstant) => {
        //        // Next byte is the constant index
        //        if offset + 1 < self.code.len() {
        //            let constant_index = self.code[offset + 1];
        //            let value = self.values[constant_index as usize];
        //            println!("OP_CONSTANT {} (value = {})", constant_index, value);
        //            offset + 2
        //        } else {
        //            println!("OP_CONSTANT <missing operand>");
        //            offset + 1
        //        }
        //    }
        //    Some(OpCode::OpNegate) => {
        //        println!("OP_NEGATE");
        //        offset + 1
        //    }
        //    Some(OpCode::OpAdd) => {
        //        println!("OP_ADD");
        //        offset + 1
        //    }
        //    Some(OpCode::OpSubtract) => {
        //        println!("OP_SUBTRACT");
        //        offset + 1
        //    }
        //    Some(OpCode::OpMultiply) => {
        //        println!("OP_MULTIPLY");
        //        offset + 1
        //    }
        //    Some(OpCode::OpDivide) => {
        //        println!("OP_DIVIDE");
        //        offset + 1
        //    }
        //    None => {
        //        println!("Unknown opcode {}", instruction);
        //        offset + 1
        //    }
        //}
    //}



