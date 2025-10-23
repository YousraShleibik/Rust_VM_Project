use rust_vm_project::{Chunk, OpCode, opcode_to_u8};
use rust_vm_project::{VirtualMachine};
use rust_vm_project::{InterpretResult};
use std::env;
use std::fs;

fn main() {

    println!("creating a bytecode chunk");
    let mut chunk = Chunk::init_chunk();
    let l = 1;

    // push 15, 42; add => 57
    let c15 = chunk.add_constant(15);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l);
    chunk.write_to_chunk(c15, l);

    let c42 = chunk.add_constant(42);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l);
    chunk.write_to_chunk(c42, l);

    chunk.write_to_chunk(opcode_to_u8(OpCode::OpAdd), l);

    // *2
    let c2 = chunk.add_constant(2);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l);
    chunk.write_to_chunk(c2, l);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpMultiply), l);

    // -5
    let c5 = chunk.add_constant(5);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l);
    chunk.write_to_chunk(c5, l);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpSubtract), l);

    // /4
    let c4 = chunk.add_constant(4);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l);
    chunk.write_to_chunk(c4, l);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpDivide), l);

    // %5, negate, return
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l);
    chunk.write_to_chunk(c5, l); // reuse 5
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpModulo), l);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpNegate), l);
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

    // Disassemble for visibility
    chunk.disassemble("demo chunk");

    // VM initial state
    let mut vm = VirtualMachine::init_machine();
    println!("chunk: {:?}", vm.chunk); // None
    println!("ip: {}", vm.ip);         // 0
    println!("stack: {:?}", vm.stack); // []

    // Run
    let result = vm.interpret(chunk);
    println!("Interpret result: {:?}", result);

    // VM state after run
    println!("chunk: {:?}", vm.chunk); // Some(Chunk { ... })
    println!("ip: {}", vm.ip);
    println!("stack: {:?}", vm.stack);

    if result == InterpretResult::InterpretSuccess {
        if let Some(top) = vm.stack.last() {
            println!("Top of stack (expected -2 if Value=i64) = {}", top);
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
