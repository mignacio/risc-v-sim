pub mod registers;
//RV32I instruction set functions

pub fn add(destination: i32, source1: i32, source2: i32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 + operand2);
}

pub fn sub(destination: i32, source1: i32, source2: i32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 - operand2);
}

pub fn and(destination: i32, source1: i32, source2: i32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 & operand2);
}

pub fn or(destination: i32, source1: i32, source2: i32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 | operand2);
}

pub fn xor(destination: i32, source1: i32, source2: i32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 ^ operand2);
}

pub fn sll(destination: i32, source1: i32, source2: i32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    // Logical shift left source1 by the value stored in the lowest 5 bits of source2.
    register::set(destination, operand1 << (openrand2 & 0x20));
}

pub fn srl(destination: i32, source1: i32, source2: i32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    // Logical shift right source1 by the value stored in the lowest 5 bits of source2.
    register::set(destination, operand1 >> (openrand2 & 0x20));
}

pub fn sra(destination: i32, source1: i32, source2: i32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    // Arithmetic shift right source1 by the value stored in the lowest 5 bits of source2.
    //TODO: Rust seems to chose arithmetic or logic shift automatically. So I think both source needs to be signed.
    register::set(destination, operand1 >> (openrand2 & 0x20));
}

pub fn slt(destination: i32, source1: i32, source2: i32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    
    let result = if(operand1 < openrand2){ 1 } else { 0};
    register::set(destination, result);
}

pub fn sltu(destination: i32, source1: u32, source2: u32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    
    let result = if(operand1 < openrand2){ 1 } else { 0};
    register::set(destination, result);
}

pub fn addi(destination: i32, source1: i32, inmediate: i32){
    let operand1 = registers::get(source1);
    register::set(destination, operand1 + inmediate);
}

pub fn andi(destination: i32, source1: i32, inmediate: i32){
    let operand1 = registers::get(source1);
    register::set(destination, operand1 & inmediate);
}

pub fn ori(destination: i32, source1: i32, inmediate: i32){
    let operand1 = registers::get(source1);
    register::set(destination, operand1 | inmediate);
}
pub fn xori(destination: i32, source1: i32, inmediate: i32){
    let operand1 = registers::get(source1);
    register::set(destination, operand1 ^ inmediate);
}
pub fn slli(){}
pub fn srli(){}
pub fn srai(){}
pub fn slti(){}
pub fn sltiu(){}

pub fn beq(){}
pub fn bne(){}
pub fn bge(){}
pub fn bgeu(){}
pub fn blt(){}
pub fn bltu(){}
pub fn jal(){}
pub fn jalr(){}
pub fn ecall(){}
pub fn ebreak(){}

pub fn lb(){}
pub fn lbu(){}
pub fn lh(){}
pub fn lhu(){}
pub fn lw(){}
pub fn lui(){}
pub fn auipc(){}
pub fn fence(){}

pub fn sb(){}
pub fn sh(){}
pub fn sw(){}

// In the future, functions for RISC ISA extensions will be defined below.