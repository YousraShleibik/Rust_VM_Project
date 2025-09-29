use rust_vm_project::{Chunk, OpCode, opcode_to_u8};

fn main() {

    println!("Hello, world!");

    println!("creating a bytecode chunk");
    let mut chunk = Chunk::init_chunk();

    let con1 = chunk.add_constant(15);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 155);
    chunk.write_to_chunk(con1, 155);
    


    let cons: u8 = chunk.add_constant(42);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 123);
    chunk.write_to_chunk(cons, 123);
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 123);

    let cons2: u8 = chunk.add_constant(45);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 124);
    chunk.write_to_chunk(cons2, 124);
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 124);
   



     chunk.disassemble("demo chunk");


}
