pub type Value = u8;

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

pub struct VirtualMachine {
    pub chunk: Option<Chunk>,
    pub ip: usize,
    pub stack: Vec<Value>,
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
        let i0 = c.add_constant(15);
        let i1 = c.add_constant(42);
        assert_eq!(i0, 0);
        assert_eq!(i1, 1);
        assert_eq!(c.values[i0 as usize], 15);
        assert_eq!(c.values[i1 as usize], 42);

        // Write opcode + operand pairs, then a Return.
        c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 10);
        c.write_to_chunk(i0, 10);

        c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 11);
        c.write_to_chunk(i1, 11);

        c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 12);

        // code and lines should be the same length
        assert_eq!(c.code.len(), c.lines.len());

        // Lines should match the writes above (per byte).
        assert_eq!(c.lines, vec![10, 10, 11, 11, 12]);

        // Opcodes should be in expected positions.
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
        let i0 = c.add_constant(10);
        c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 1);
        c.write_to_chunk(i0, 1);

        let i1 = c.add_constant(20);
        c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 2);
        c.write_to_chunk(i1, 2);

        c.write_to_chunk(opcode_to_u8(OpCode::OpAdd), 3);

        // Unknown opcode (should still advance by 1)
        c.write_to_chunk(0xFF, 4);

        c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 5);

        // Offsets should advance: 0->2 (constant), 2->4 (constant), 4->5 (add), 5->6 (unknown), 6->7 (return)
        let mut off = 0usize;
        off = c.disassemble_instruction(off);
        assert_eq!(off, 2);

        off = c.disassemble_instruction(off);
        assert_eq!(off, 4);

        off = c.disassemble_instruction(off);
        assert_eq!(off, 5);

        off = c.disassemble_instruction(off);
        assert_eq!(off, 6);

        off = c.disassemble_instruction(off);
        assert_eq!(off, 7);

        // Sanity check lines length matches code length.
        assert_eq!(c.code.len(), c.lines.len());
        assert_eq!(c.lines, vec![1, 1, 2, 2, 3, 4, 5]);
    }
}



