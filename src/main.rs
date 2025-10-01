use rust_vm_project::{Chunk, OpCode, opcode_to_u8};
use rust_vm_project::{VirtualMachine, Value};
use rust_vm_project::{InterpretResult};

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
   
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpAdd), 200);       // 15 + 42
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 201);  // push 45
    chunk.write_to_chunk(cons2, 201);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpMultiply), 202);  // (15+42)*45

    chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 250);



     chunk.disassemble("demo chunk");

    let mut vm = VirtualMachine::init_machine();
    println!("chunk: {:?}", vm.chunk);
    println!("ip: {}", vm.ip);
    println!("stack: {:?}", vm.stack);


    let mut vm = VirtualMachine::init_machine();
    let result = vm.interpret(chunk);
    println!("Interpret result: {:?}", result);
    println!("Final stack: {:?}", vm.stack);

}

