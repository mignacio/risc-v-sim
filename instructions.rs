pub mod registers;
//RV32I instruction set functions
#[allow(dead_code)]
pub fn add(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 + operand2);
}

#[allow(dead_code)]
pub fn sub(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 - operand2);
}

#[allow(dead_code)]
pub fn and(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 & operand2);
}

#[allow(dead_code)]
pub fn or(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 | operand2);
}

#[allow(dead_code)]
pub fn xor(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    registers::set(destination, operand1 ^ operand2);
}

#[allow(dead_code)]
pub fn sll(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    // Logical shift left source1 by the value stored in the lowest 5 bits of source2.
    registers::set(destination, operand1 << (operand2 & 0x20));
}

#[allow(dead_code)]
pub fn srl(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    // Logical shift right source1 by the value stored in the lowest 5 bits of source2.
    registers::set(destination, operand1 >> (operand2 & 0x20));
}

#[allow(dead_code)]
pub fn sra(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    // Arithmetic shift right source1 by the value stored in the lowest 5 bits of source2.
    //TODO: Rust seems to chose arithmetic or logic shift automatically. So I think both source needs to be signed.
    registers::set(destination, operand1 >> (operand2 & 0x20));
}

#[allow(dead_code)]
pub fn slt(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    
    let result = if operand1 < operand2 { 1 } else { 0};
    registers::set(destination, result);
}

#[allow(dead_code)]
pub fn sltu(destination: usize, source1: usize, source2: usize){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    
    let result = if operand1 < operand2 { 1 } else { 0};
    registers::set(destination, result);
}

#[allow(dead_code)]
pub fn addi(destination: usize, source1: usize, immediate: i32){
    let operand1 = registers::get(source1);
    registers::set(destination, operand1 + immediate);
}

#[allow(dead_code)]
pub fn andi(destination: usize, source1: usize, immediate: i32){
    let operand1 = registers::get(source1);
    registers::set(destination, operand1 & immediate);
}

#[allow(dead_code)]
pub fn ori(destination: usize, source1: usize, immediate: i32){
    let operand1 = registers::get(source1);
    registers::set(destination, operand1 | immediate);
}

#[allow(dead_code)]
pub fn xori(destination: usize, source1: usize, immediate: i32){
    let operand1 = registers::get(source1);
    registers::set(destination, operand1 ^ immediate);
}

#[allow(dead_code)]
pub fn slli(destination: usize, source1: usize, immediate: i32){
    let operand1 = registers::get(source1);
    registers::set(destination, operand1 << (immediate & 0x20));
}

#[allow(dead_code)]
pub fn srli(destination: usize, source1: usize, immediate: i32){
    let operand1 = registers::get(source1);
    registers::set(destination, operand1 >> (immediate & 0x20));
}

#[allow(dead_code)]
pub fn srai(destination: usize, source1: usize, immediate: i32){
    let operand1 = registers::get(source1);
    registers::set(destination, operand1 >> (immediate & 0x20));
}

#[allow(dead_code)]
pub fn slti(destination: usize, source1: usize, immediate: i32){
    let operand1 = registers::get(source1);
    
    let result = if operand1 < immediate { 1 } else { 0};
    registers::set(destination, result);
}

#[allow(dead_code)]
pub fn sltiu(destination: usize, source1: usize, immediate: u32){
    let operand1 = registers::get(source1) as u32;
    
    let result = if operand1 < immediate { 1 } else { 0};
    registers::set(destination, result);
}

#[allow(dead_code)]
pub fn beq(source1: usize, source2: usize, offset: u32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    if operand1 == operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += offset;
        }
    }
}

#[allow(dead_code)]
pub fn bne(source1: usize, source2: usize, offset: u32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    if operand1 != operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += offset;
        }
    }
}

#[allow(dead_code)]
pub fn bge(source1: usize, source2: usize, offset: u32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    if operand1 >= operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += offset;
        }
    }
}

#[allow(dead_code)]
pub fn bgeu(source1: usize, source2: usize, offset: u32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    if operand1 == operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += offset;
        }
    }
}

#[allow(dead_code)]
pub fn blt(source1: usize, source2: usize, offset: u32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    if operand1 < operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += offset;
        }
    }
}

#[allow(dead_code)]
pub fn bltu(source1: usize, source2: usize, offset: u32){
    let operand1 = registers::get(source1);
    let operand2 = registers::get(source2);
    if operand1 < operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += offset;
        }
    }
}

#[allow(dead_code)]
pub fn jal(source1: usize, offset: u32){
    unsafe{
        registers::PROGRAM_COUNTER += offset;
    }
    registers::set(source1, registers::get(source1) + 4);
}

#[allow(dead_code)]
pub fn jalr(){
    //TODO
}

#[allow(dead_code)]
pub fn ecall(){
    //TODO: Raises EnvironmentCall
}

#[allow(dead_code)]
pub fn ebreak(){
    //TODO: Raises Breakpoint
}

#[allow(dead_code)]
pub fn lb(){}

#[allow(dead_code)]
pub fn lbu(){}

#[allow(dead_code)]
pub fn lh(){}

#[allow(dead_code)]
pub fn lhu(){}

#[allow(dead_code)]
pub fn lw(){}

#[allow(dead_code)]
pub fn lui(){}

#[allow(dead_code)]
pub fn auipc(){}

#[allow(dead_code)]
pub fn fence(){}

#[allow(dead_code)]
pub fn sb(){}

#[allow(dead_code)]
pub fn sh(){}

#[allow(dead_code)]
pub fn sw(){}

// In the future, functions for RISC ISA extensions will be defined below.