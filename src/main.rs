use rust_vm_project::{Chunk, OpCode, opcode_to_u8};
use rust_vm_project::{VirtualMachine};
use rust_vm_project::{InterpretResult};
use std::env;
use std::fs;
use rust_vm_project::Value;


fn main() {

    println!("creating a bytecode chunk");
    let mut chunk = Chunk::init_chunk();
    let l = 1;

    let c15 = chunk.add_constant(Value::ValNumber(15));
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l);
    chunk.write_to_chunk(c15, l);

    let c42 = chunk.add_constant(Value::ValNumber(42));
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
    println!("chunk: {:?}", vm.chunk); 
    println!("ip: {}", vm.ip);         
    println!("stack: {:?}", vm.stack); 

    let result = vm.interpret_chunk(chunk);
    println!("Interpret result: {:?}", result);

    // VM state after run
    println!("chunk: {:?}", vm.chunk); // Some(Chunk { ... })
    println!("ip: {}", vm.ip);
    println!("stack: {:?}", vm.stack);

    if result == InterpretResult::InterpretSuccess {
        if let Some(top) = vm.stack.last() {
            println!("Top of stack (expected -2) = {:?}", top);
        }
    }

// compiler/parser demos 
if let Some(flag) = env::args().nth(1) {
    if flag == "--compile" {
        // Usage: cargo run -- --compile "1+2*3"
        let expr = env::args()
            .nth(2)
            .expect("Usage: cargo run -- --compile \"<expr>\"");
        let mut vm = VirtualMachine::init_machine();
        let result = vm.interpret(&expr);
        println!("Interpret result: {:?}", result);
        if let Some(c) = &vm.chunk {
            c.disassemble("compiled (expr)");
        }
        println!("Final stack: {:?}", vm.stack);
        return; 
    } else if flag == "--compile-file" {
        let path = env::args()
            .nth(2)
            .expect("Usage: cargo run -- --compile-file <file.lox>");
        let source = std::fs::read_to_string(&path).expect("Failed to read source file");
        let mut vm = VirtualMachine::init_machine();
        let result = vm.interpret(&source);
        println!("Interpret result: {:?}", result);
        if let Some(c) = &vm.chunk {
            c.disassemble("compiled (file)");
        }
        println!("Final stack: {:?}", vm.stack);
        return; 
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

    if let Some(flag) = env::args().nth(1) {
    if flag == "--demo-bool" {
        {
            let mut c = Chunk::init_chunk();
            let l = 1u8;
            c.write_to_chunk(opcode_to_u8(OpCode::OpTrue), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpNot), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

            println!("\n-- demo: !true --");
            c.disassemble("bool/not (true)");
            let mut vm = VirtualMachine::init_machine();
            let res = vm.interpret_chunk(c);
            println!("Interpret result: {:?}", res);
            println!("Final stack: {:?}", vm.stack);
        }

                {
            let mut c = Chunk::init_chunk();
            let l = 1u8;
            c.write_to_chunk(opcode_to_u8(OpCode::OpNil), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpNot), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

            println!("\n-- demo: !nil --");
            c.disassemble("bool/not (nil)");
            let mut vm = VirtualMachine::init_machine();
            let res = vm.interpret_chunk(c);
            println!("Interpret result: {:?}", res);
            println!("Final stack: {:?}", vm.stack);
        }

                {
            let mut c = Chunk::init_chunk();
            let l = 1u8;
            let i3 = c.add_constant(Value::ValNumber(3));
            c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(i3, l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpNot), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

            println!("\n-- demo: !3 --");
            c.disassemble("bool/not (number)");
            let mut vm = VirtualMachine::init_machine();
            let res = vm.interpret_chunk(c);
            println!("Interpret result: {:?}", res);
            println!("Final stack: {:?}", vm.stack);
        }

         return;
    } else if flag == "--demo-cmp" {
        // 3 == 3 -> true
        {
            let mut c = Chunk::init_chunk();
            let l = 1u8;
            let a = c.add_constant(Value::ValNumber(3));
            let b = c.add_constant(Value::ValNumber(3));
            c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(a, l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(b, l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpEqual), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

            println!("\n-- demo: 3 == 3 --");
            c.disassemble("cmp (==)");
            let mut vm = VirtualMachine::init_machine();
            let res = vm.interpret_chunk(c);
            println!("Interpret result: {:?}", res);
            println!("Final stack: {:?}", vm.stack);
        }

        // 3 != 4 -> true  (lowered to Equal; Not)
        {
            let mut c = Chunk::init_chunk();
            let l = 1u8;
            let a = c.add_constant(Value::ValNumber(3));
            let b = c.add_constant(Value::ValNumber(4));
            c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(a, l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(b, l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpEqual), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpNot), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

            println!("\n-- demo: 3 != 4 --");
            c.disassemble("cmp (!=)");
            let mut vm = VirtualMachine::init_machine();
            let res = vm.interpret_chunk(c);
            println!("Interpret result: {:?}", res);
            println!("Final stack: {:?}", vm.stack);
        }

        // 3 >= 2 -> true  (lowered to Less; Not)
        {
            let mut c = Chunk::init_chunk();
            let l = 1u8;
            let a = c.add_constant(Value::ValNumber(3));
            let b = c.add_constant(Value::ValNumber(2));
            c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(a, l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(b, l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpLess), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpNot), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

            println!("\n-- demo: 3 >= 2 --");
            c.disassemble("cmp (>== lowered)");
            let mut vm = VirtualMachine::init_machine();
            let res = vm.interpret_chunk(c);
            println!("Interpret result: {:?}", res);
            println!("Final stack: {:?}", vm.stack);
        }

        // 2 <= 1 -> false (lowered to Greater; Not)
        {
            let mut c = Chunk::init_chunk();
            let l = 1u8;
            let a = c.add_constant(Value::ValNumber(2));
            let b = c.add_constant(Value::ValNumber(1));
            c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(a, l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpConstant), l); c.write_to_chunk(b, l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpGreater), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpNot), l);
            c.write_to_chunk(opcode_to_u8(OpCode::OpReturn), l);

            println!("\n-- demo: 2 <= 1 --");
            c.disassemble("cmp (<= lowered)");
            let mut vm = VirtualMachine::init_machine();
            let res = vm.interpret_chunk(c);
            println!("Interpret result: {:?}", res);
            println!("Final stack: {:?}", vm.stack);
        }
        return;
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
