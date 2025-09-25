use rust_vm_project::{Chunk, OpCode, opcode_to_u8};

fn main() {

    println!("Hello, world!");

    println!("creating a bytecode chunk");
    let  mut chunk: Chunk = Chunk::init_chunk();


    let cons: u8 = chunk.add_constant(42);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 123);
    //chunk.write_to_chunk(cons, 123);
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 123);

    chunk.dump();
}
