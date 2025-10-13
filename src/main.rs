use rust_vm_project::{Chunk, OpCode, opcode_to_u8};
use rust_vm_project::{VirtualMachine};
use rust_vm_project::{InterpretResult};
use std::env;
use std::fs;

fn main() {

    println!("Hello, world!");

    println!("creating a bytecode chunk");
    let mut chunk = Chunk::init_chunk();
    let l = 1;

    let c15 = chunk.add_constant(15);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l);
    chunk.write_to_chunk(c15, l);

    let c42 = chunk.add_constant(42);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l);
    chunk.write_to_chunk(c42, l);

    chunk.write_to_chunk(opcode_to_u8(OpCode::OpAdd), l); // 15 + 42
     chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

    // Disassemble & run
    chunk.disassemble("demo chunk");

    let mut vm = VirtualMachine::init_machine();
    println!("chunk: {:?}", vm.chunk); 
    println!("ip: {}", vm.ip);         
    println!("stack: {:?}", vm.stack); 

    let result = vm.interpret(chunk);
    println!("Interpret result: {:?}", result);
    println!("chunk: {:?}", vm.chunk); 
    println!("ip: {}", vm.ip);
    println!("stack: {:?}", vm.stack);
    if result == InterpretResult::InterpretSuccess {
        if let Some(top) = vm.stack.last() {
            println!("Top of stack (expected -2) = {}", top);
        }
    }



    if let Some(flag) = env::args().nth(1) {
    if flag == "--scan" {
        let path = env::args()
            .nth(2)
            .expect("Usage: cargo run -- --scan <file.lox>");
        let source = fs::read_to_string(&path).expect("Failed to read source file");
        let mut vm = VirtualMachine::init_machine();
        let result = vm.interpret_source(&source);
        println!("Interpret result: {:?}", result);
        }
    }
}
    


    //let cons: u8 = chunk.add_constant(42);
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 123);
    //chunk.write_to_chunk(cons, 123);
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 123);

    //let cons2: u8 = chunk.add_constant(45);
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 124);
    //chunk.write_to_chunk(cons2, 124);
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 124);
   
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpAdd), 200);       // 15 + 42
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 201);  // push 45
    //chunk.write_to_chunk(cons2, 201);
    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpMultiply), 202);  // (15+42)*45

    //chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 250);



     //chunk.disassemble("demo chunk");

    //let mut vm = VirtualMachine::init_machine();
    //println!("chunk: {:?}", vm.chunk);
    //println!("ip: {}", vm.ip);
    //println!("stack: {:?}", vm.stack);


    //let mut vm = VirtualMachine::init_machine();
    //let result = vm.interpret(chunk);
    //println!("Interpret result: {:?}", result);
    //println!("Final stack: {:?}", vm.stack);

//}
