pub mod registers;
extern crate phf;
use phf::phf_map;

pub type InstFunc = fn(RV32I);

#[allow(dead_code)]
pub static OPCODES: phf::Map<i32, InstFunc> = phf_map!{
    19i32 => addi,
};

// Struct for decoded instruction.
pub struct RV32I{
    raw_inst:   i32, // Raw instruction.
    pub opcode:     i32, // Opcode field.
    rs1:        i32, // Source register 1.
    rd:         i32, // Destination register.
    imm:        i32, // Sign extended immediate value for I type.
    funct3:     i32 // Sub-function value I type.
}

impl RV32I {
    pub fn decode(coded_inst: i32) -> RV32I{
        RV32I {
            raw_inst: coded_inst,
            opcode: (coded_inst & 0b0000000000000000000000001111111),
            rd:     (coded_inst & 0b0000000000000000000111110000000) >> 7,
            funct3: (coded_inst & 0b0000000000000000111000000000000) >> 12,
            rs1:    (coded_inst & 0b0000000000011111000000000000000) >> 15,
            imm:    (coded_inst & 0b1111111111100000000000000000000) >> 20,
        }
    }
    // Function to print the decoded instruction in a human readable way.
    #[allow(dead_code)]
    pub fn to_string(&self) -> String{
        format!("raw_inst: {:#032b}\n\
                 opcode:   {:#032b}\n\
                 rd:       {:#032b}\n\
                 rs1:      {:#032b}\n\
                 imm:      {:#032b}\n\
                 funct3:   {:#032b}\n", self.raw_inst, self.opcode, self.rd, self.rs1, self.imm, self.funct3)
    
    }
}

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
pub fn addi(instruction: RV32I){
    let rs1 = registers::get(instruction.rs1.try_into().unwrap());
    registers::fake_set(instruction.rd, rs1 + instruction.imm);
    println!("ADDI");
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