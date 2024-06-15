pub mod registers;

pub mod instruction_types;

extern crate phf;
use phf::phf_map;

pub type InstFunc = fn(instruction_types::DecodedInst);

#[allow(dead_code)]
pub static I_CODES: phf::Map<i32, InstFunc> = phf_map!{
    0x0i32 => addi,
    0x4i32 => xori,
    0x6i32 => ori,
    0x7i32 => andi,
    0x1i32 => slli,
    0x5i32 => srai,
    0x2i32 => slti,
    0x3i32 => sltiu
};

//RV32I instruction set functions sorted by opcode.
// TYPE R - OPCODE 0110011
#[allow(dead_code)]
pub fn add(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    registers::set(instruction.rd, operand1 + operand2);
}
#[allow(dead_code)]
pub fn sub(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    registers::set(instruction.rd, operand1 - operand2);
}
#[allow(dead_code)]
pub fn xor(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    registers::set(instruction.rd, operand1 ^ operand2);
}
#[allow(dead_code)]
pub fn or(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    registers::set(instruction.rd, operand1 | operand2);
}
#[allow(dead_code)]
pub fn and(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    registers::set(instruction.rd, operand1 & operand2);
}
#[allow(dead_code)]
pub fn sll(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    // Logical shift left source1 by the value stored in the lowest 5 bits of source2.
    registers::set(instruction.rd, operand1 << (operand2 & 0x20));
}
#[allow(dead_code)]
pub fn srl(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    // Logical shift right source1 by the value stored in the lowest 5 bits of source2.
    registers::set(instruction.rd, operand1 >> (operand2 & 0x20));
}
#[allow(dead_code)]
pub fn sra(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    // Arithmetic shift right source1 by the value stored in the lowest 5 bits of source2.
    //TODO: Rust seems to chose arithmetic or logic shift automatically. So I think both source needs to be signed.
    registers::set(instruction.rd, operand1 >> (operand2 & 0x20));
}
#[allow(dead_code)]
pub fn slt(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    
    let result = if operand1 < operand2 { 1 } else { 0};
    registers::set(instruction.rd, result);
}
#[allow(dead_code)]
pub fn sltu(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    
    let result = if operand1 < operand2 { 1 } else { 0};
    registers::set(instruction.rd, result);
}

// TYPE - I OPCODE 0010011
#[allow(dead_code)]
pub fn addi(instruction: instruction_types::DecodedInst){
    let rs1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, rs1 + instruction.imm);
}
#[allow(dead_code)]
pub fn xori(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1.try_into().unwrap());
    registers::set(instruction.rd, operand1 ^ instruction.imm);
}
#[allow(dead_code)]
pub fn andi(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 & instruction.imm);
}
#[allow(dead_code)]
pub fn ori(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 | instruction.imm);
}
#[allow(dead_code)]
pub fn slli(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 << (instruction.imm & 0x20));
}
#[allow(dead_code)]
pub fn srli(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 >> (instruction.imm & 0x20));
}
#[allow(dead_code)]
pub fn srai(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 >> (instruction.imm & 0x20));
}
#[allow(dead_code)]
pub fn slti(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    
    let result = if operand1 < instruction.imm { 1 } else { 0};
    registers::set(instruction.rd, result);
}
#[allow(dead_code)]
pub fn sltiu(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1) as u32;
    
    let result = if operand1 < instruction.imm as u32 { 1 } else { 0};
    registers::set(instruction.rd, result);
}

// TYPE I - OPCODE 00000011
#[allow(dead_code)]
pub fn lb(){}
#[allow(dead_code)]
pub fn lh(){}
#[allow(dead_code)]
pub fn lw(){}
#[allow(dead_code)]
pub fn lbu(){}
#[allow(dead_code)]
pub fn lhu(){}


// TYPE S - OPCODE 0100011
#[allow(dead_code)]
pub fn sb(){}
#[allow(dead_code)]
pub fn sh(){}
#[allow(dead_code)]
pub fn sw(){}

// TYPE B - OPCODE 1100011
#[allow(dead_code)]
pub fn beq(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    if operand1 == operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += instruction.imm as u32;
        }
    }
}
#[allow(dead_code)]
pub fn bne(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    if operand1 != operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += instruction.imm as u32;
        }
    }
}
#[allow(dead_code)]
pub fn blt(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    if operand1 < operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += instruction.imm as u32;
        }
    }
}
#[allow(dead_code)]
pub fn bge(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    if operand1 >= operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += instruction.imm as u32;
        }
    }
}
#[allow(dead_code)]
pub fn bltu(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    if operand1 < operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += instruction.imm as u32;
        }
    }
}
#[allow(dead_code)]
pub fn bgeu(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    let operand2 = registers::get(instruction.rs2);
    if operand1 == operand2
    {
        unsafe{
            registers::PROGRAM_COUNTER += instruction.imm as u32;
        }
    }
}

// TYPE J/I - OPCODE 1101111
#[allow(dead_code)]
pub fn jal(instruction: instruction_types::DecodedInst){
    unsafe{
        registers::PROGRAM_COUNTER += instruction.imm as u32;
    }
    registers::set(instruction.rs1, registers::get(instruction.rs1) + 4);
}
#[allow(dead_code)]
pub fn jalr(){
    //TODO
}

// TYPE U - OPCODE 0110111
#[allow(dead_code)]
pub fn lui(){}
#[allow(dead_code)]
pub fn auipc(){}

// TYPE I - OPCODE 1110011
#[allow(dead_code)]
pub fn ecall(){
    //TODO: Raises EnvironmentCall
}
#[allow(dead_code)]
pub fn ebreak(){
    //TODO: Raises Breakpoint
}

// OPCODE 0000111 (TODO: confirm opcode for this instruction)
#[allow(dead_code)]
pub fn fence(){}

// In the future, functions for RISC ISA extensions will be defined below.