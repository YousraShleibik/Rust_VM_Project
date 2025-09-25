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