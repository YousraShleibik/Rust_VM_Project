use rust_vm_project::{VirtualMachine, Value, InterpretResult, Number};

/// Run a Lox expression string through the VM and return result + final stack.
fn run_program(program: &str) -> (InterpretResult, Vec<Value>) {
    let mut vm = VirtualMachine::init_machine();
    let result = vm.interpret(program);
    (result, vm.stack.clone())
}

/// Evaluate a program that is expected to succeed and return its numeric value.
fn eval_ok(program: &str) -> Number {
    let (result, stack) = run_program(program);

    assert_eq!(
        result,
        InterpretResult::InterpretSuccess,
        "VM did not succeed for program `{}`; got {:?}",
        program,
        result
    );

    match stack.last().copied() {
        Some(Value::ValNumber(n)) => n,
        other => panic!("Expected numeric result on stack, got {:?} for program `{}`", other, program),
    }
}

/// check numeric result.
fn check(program: &str, expected: Number) {
    let got = eval_ok(program);
    assert_eq!(got, expected, "program: `{}`", program);
}

/// Expect a runtime error (e.g. divide by zero).
fn expect_runtime_error(program: &str) {
    let (result, _stack) = run_program(program);
    assert_eq!(
        result,
        InterpretResult::InterpretRuntimeError,
        "expected runtime error for `{}`, got {:?}",
        program,
        result
    );
}

/// Expect a compile error (e.g. bad syntax like `2++3`).
fn expect_compile_error(program: &str) {
    let (result, _stack) = run_program(program);
    assert_eq!(
        result,
        InterpretResult::InterpretCompileError,
        "expected compile error for `{}`, got {:?}",
        program,
        result
    );
}


#[test]
fn single_number() {
    check("42", 42);
}

// Addition
#[test]
fn add_simple() {
    check("1+8", 9);
}
#[test]
fn add_with_zero() {
    check("10+0", 10);
}
#[test]
fn add_two_digits() {
    check("7+5", 12);
}

// Multiplication
#[test]
fn mul_simple() {
    check("3*4", 12);
}
#[test]
fn mul_by_zero() {
    check("5*0", 0);
}
#[test]
fn mul_two_digits() {
    check("7*6", 42);
}

// Subtraction
#[test]
fn sub_simple() {
    check("5-3", 2);
}
#[test]
fn sub_negative_result() {
    check("0-7", -7);
}
#[test]
fn sub_two_digits() {
    check("20-5", 15);
}

// Division (integer)
#[test]
fn div_simple() {
    check("6/2", 3);
}
#[test]
fn div_exact() {
    check("9/3", 3);
}
#[test]
fn div_two_digits() {
    check("20/5", 4);
}

// Mixed precedence and parentheses
#[test]
fn mix_precedence() {
    // 2 + 3*4 = 14
    check("2+3*4", 14);
}
#[test]
fn mix_parentheses_change_precedence() {
    check("(2+3)*4", 20);
}
#[test]
fn mix_nested_parentheses() {
    // 18/(3*(1+2)) = 18/9 = 2
    check("18/(3*(1+2))", 2);
}

// Whitespace robustness
#[test]
fn spaces_everywhere() {
    check("   2   +    3   *   4   ", 14);
}
#[test]
fn tabs_and_newlines() {
    check("\t(\n 2 + 3 \n)\n * \n4\t", 20);
}
#[test]
fn redundant_parentheses() {
    check("((((42))))", 42);
}

// Precedence traps
#[test]
fn chain_mul_then_add() {
    // (2*3*4)=24; +5=29
    check("2*3*4+5", 29);
}
#[test]
fn add_then_mul_chain() {
    // 3*4*5=60; +2=62
    check("2+3*4*5", 62);
}
#[test]
fn mixed_with_parentheses() {
    // 1+2*3-4/2+(6*(7+8))
    // 1+6-2+6*15 = 1+6-2+90 = 95
    check("1+2*3-4/2+(6*(7+8))", 95);
}

// Negative numbers & unary minus
#[test]
fn leading_negative_single() {
    check("-7", -7);
}
#[test]
fn negative_plus_positive() {
    check("-7+2", -5);
}
#[test]
fn negative_times_positive() {
    check("-3*4", -12);
}
#[test]
fn paren_then_unary_minus_on_group() {
    check("-(2+3)*4", -20);
}
#[test]
fn nested_negations() {
    check("-(-(-5))", -5);
}

// Division behavior
#[test]
fn integer_division_truncates() {
    check("10/3", 3);
}
#[test]
fn negative_division_truncates_toward_zero() {
    check("-7/2", -3); // -3.5 -> -3
}
#[test]
fn mixed_add_div_neg() {
    // (-7/2)=-3; 10+(-3)=7
    check("10+ -7/2", 7);
}

// Zero interactions
#[test]
fn add_zero_noop() {
    check("123+0", 123);
}
#[test]
fn mul_by_zero_kills_term2() {
    check("999*0+7", 7);
}
#[test]
fn zero_minus_positive() {
    check("0-9", -9);
}

// Larger multi-digit & nesting depth
#[test]
fn deep_nesting_corrected() {
    // 12 + (34*5) = 182
    // (8*2)=16; 7+16=23; 5+23=28; 6*28=168
    // 182 - 168 = 14
    check("((12+(34*5))-(6*(5+(7+(8*2)))))", 14);
}

// Parentheses that don't change value
#[test]
fn neutral_parens() {
    check("(1)+(2)", 3);
}

// Long flat expression
#[test]
fn long_flat_mix() {
    // 1+2+3*4*5-6/2+7*2 = 1+2+60-3+14 = 74
    check("1+2+3*4*5-6/2+7*2", 74);
}

// Whitespace-only around a number
#[test]
fn number_with_lots_of_space() {
    check("     31415     ", 31415);
}

// Left-associativity for +/-
#[test]
fn left_assoc_add_sub() {
    // (10-2)+3 = 11; wrong grouping would give 5
    check("10-2+3", 11);
}

// Left-associativity for *//
#[test]
fn left_assoc_mul_div() {
    // (20/5)*2 = 8; wrong grouping 20/(5*2)=2
    check("20/5*2", 8);
}

#[test]
fn div_by_zero_runtime_error() {
    expect_runtime_error("1/0");
}

#[test]
fn invalid_double_plus_is_compile_error() {
    expect_compile_error("2++3");
}

