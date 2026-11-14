
pub struct Registers {
    regs: [u32; 32],
    program_counter: u32,
}

impl Registers {
    fn print(&self) {
        for reg in self.regs{
            println!{"{:#08X}", reg};
        }
        println!("PC: {:#08X}", self.program_counter);
    }

    fn set(&mut self, index: usize, val: u32) {
        // Register x0 is always 0, cannot be set to a different value.
        if index != 0 {
            self.regs[index] = val;
        }
    }

    fn get(&self, index:usize) -> u32 {
        self.regs[index]
    }

    fn clear(&mut self, index: usize) {
        self.regs[index] = 0;
    }

    fn step(&mut self) {
        self.program_counter+=1;
    }
}

pub struct Cpu {
    regs: Registers,
    memory: Vec<u32>,
}

pub enum CpuFault {
    MisalignedAddress,
    AddressOutOfBounds,
    DecodeError,
    ExecuteError,
}


const MEM_SIZE: u32 = 0xFFFFFFFF; //Placeholder for now
impl Cpu {
    pub fn reset(&mut self) {

    }

    pub fn step(&mut self) -> Result<u32, CpuFault> {
        let fetch_res: u32 = self.fetch(self.regs.program_counter)?;
        let decoded: DecodedInst = self.decode(fetch_res)?;
        self.execute(decoded);
        self.regs.step();
        Ok(fetch_res)
    }

    fn fetch(&mut self, program_counter: u32) -> Result<u32, CpuFault> {
        // Is PC byte aligned?
        if program_counter % 4 != 0 {
            Err(CpuFault::MisalignedAddress)
        }
        // Is it inside memory boundaries?
        else if program_counter > MEM_SIZE {
            Err(CpuFault::AddressOutOfBounds)
        }
        else {
            let res:Option<&u32> = self.memory.get(program_counter as usize);
            match res {
                Some(value) => Ok(*value),
                None => Ok(0),
            }
        }
    }

    fn decode(&mut self, raw_inst: u32) -> Result<instruction_types::DecodedInst, CpuFault>{
        instruction_types::DecodedInst::decode(raw_inst)
            .map_err(|_| CpuFault::DecodeError)
    }

    fn execute(&mut self, decoded_inst: DecodedInst) {
        match decoded_inst.funct3 {
            0x0u32 => addi,
            0x4u32 => xori,
            0x6u32 => ori,
            0x7u32 => andi,
            0x1u32 => slli,
            0x5u32 => srai,
            0x2u32 => slti,
            0x3u32 => sltiu,
            8_u32..=u32::MAX => todo!()
        };
    }
}

pub fn nop(_instruction: instruction_types::DecodedInst){

}

//RV32I instruction set functions sorted by opcode.
// TYPE R - OPCODE 0110011
#[allow(dead_code)]
pub fn add(cpu: &mut Cpu){
    let operand1: u32 = registers::get(instruction.rs1);
    let operand2: u32 = registers::get(instruction.rs2);
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
    let rs1: u32 = registers::get(instruction.rs1);
    registers::set(instruction.rd, rs1 + instruction.imm);
    println!("ADDI");
}
#[allow(dead_code)]
pub fn xori(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1.try_into().unwrap());
    registers::set(instruction.rd, operand1 ^ instruction.imm);
    println!("XORI");
}
#[allow(dead_code)]
pub fn andi(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 & instruction.imm);
    println!("ANDI");
}
#[allow(dead_code)]
pub fn ori(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 | instruction.imm);
    println!("ORI");
}
#[allow(dead_code)]
pub fn slli(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 << (instruction.imm & 0x20));
    println!("SLLI");
}
#[allow(dead_code)]
pub fn srli(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 >> (instruction.imm & 0x20));
    println!("SRLI");
}
#[allow(dead_code)]
pub fn srai(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    registers::set(instruction.rd, operand1 >> (instruction.imm & 0x20));
    println!("SRLI/SRAI");
}
#[allow(dead_code)]
pub fn slti(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1);
    
    let result = if operand1 < instruction.imm { 1 } else { 0};
    registers::set(instruction.rd, result);
    println!("SLTI");
}
#[allow(dead_code)]
pub fn sltiu(instruction: instruction_types::DecodedInst){
    let operand1 = registers::get(instruction.rs1) as u32;
    
    let result = if operand1 < instruction.imm as u32 { 1 } else { 0};
    registers::set(instruction.rd, result);
    println!("SLTIU");
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