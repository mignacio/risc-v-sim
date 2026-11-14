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

    fn clear_all(&mut self) {
        for indx in 0..32 {
            self.clear(indx);
        }
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
        self.regs.clear_all();
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

    fn decode(&mut self, raw_inst: u32) -> Result<DecodedInst, CpuFault>{
        DecodedInst::decode(raw_inst)
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

    fn load_memory(&mut self, memory: Vec<u32>){
        self.memory = memory;
    }
}

pub fn nop(cpu: &mut Cpu, instruction: &DecodedInst){

}

//RV32I instruction set functions sorted by opcode.
// TYPE R - OPCODE 0110011
#[allow(dead_code)]
pub fn add(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1: u32 = cpu.regs.get(instruction.rs1 as usize);
    let operand2: u32 = cpu.regs.get(instruction.rs2 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 + operand2);
}
#[allow(dead_code)]
pub fn sub(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1: u32 = cpu.regs.get(instruction.rs1 as usize);
    let operand2: u32 = cpu.regs.get(instruction.rs2 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 - operand2);
}
#[allow(dead_code)]
pub fn xor(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1: u32 = cpu.regs.get(instruction.rs1 as usize);
    let operand2: u32 = cpu.regs.get(instruction.rs2 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 ^ operand2);
}
#[allow(dead_code)]
pub fn or(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 | operand2);
}
#[allow(dead_code)]
pub fn and(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 & operand2);
}
#[allow(dead_code)]
pub fn sll(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    // Logical shift left source1 by the value stored in the lowest 5 bits of source2.
    cpu.regs.set(instruction.rd as usize, operand1 << (operand2 & 0x20));
}
#[allow(dead_code)]
pub fn srl(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    // Logical shift right source1 by the value stored in the lowest 5 bits of source2.
    cpu.regs.set(instruction.rd as usize, operand1 >> (operand2 & 0x20));
}
#[allow(dead_code)]
pub fn sra(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    // Arithmetic shift right source1 by the value stored in the lowest 5 bits of source2.
    //TODO: Rust seems to chose arithmetic or logic shift automatically. So I think both source needs to be signed.
    cpu.regs.set(instruction.rd as usize, operand1 >> (operand2 & 0x20));
}
#[allow(dead_code)]
pub fn slt(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    
    let result = if operand1 < operand2 { 1 } else { 0};
    cpu.regs.set(instruction.rd as usize, result);
}
#[allow(dead_code)]
pub fn sltu(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    
    let result = if operand1 < operand2 { 1 } else { 0};
    cpu.regs.set(instruction.rd as usize, result);
}

// TYPE - I OPCODE 0010011
#[allow(dead_code)]
pub fn addi(cpu: &mut Cpu, instruction: &DecodedInst){
    let rs1: u32 = cpu.regs.get(instruction.rs1 as usize);
    cpu.regs.set(instruction.rd as usize, rs1 + instruction.imm);
    println!("ADDI");
}
#[allow(dead_code)]
pub fn xori(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1.try_into().unwrap());
    cpu.regs.set(instruction.rd as usize, operand1 ^ instruction.imm);
    println!("XORI");
}
#[allow(dead_code)]
pub fn andi(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 & instruction.imm);
    println!("ANDI");
}
#[allow(dead_code)]
pub fn ori(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 | instruction.imm);
    println!("ORI");
}
#[allow(dead_code)]
pub fn slli(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 << (instruction.imm & 0x20));
    println!("SLLI");
}
#[allow(dead_code)]
pub fn srli(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 >> (instruction.imm & 0x20));
    println!("SRLI");
}
#[allow(dead_code)]
pub fn srai(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    cpu.regs.set(instruction.rd as usize, operand1 >> (instruction.imm & 0x20));
    println!("SRLI/SRAI");
}
#[allow(dead_code)]
pub fn slti(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    
    let result = if operand1 < instruction.imm { 1 } else { 0};
    cpu.regs.set(instruction.rd as usize, result);
    println!("SLTI");
}
#[allow(dead_code)]
pub fn sltiu(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize) as u32;
    
    let result = if operand1 < instruction.imm as u32 { 1 } else { 0};
    cpu.regs.set(instruction.rd as usize, result);
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
pub fn beq(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    if operand1 == operand2
    {
        cpu.regs.program_counter += instruction.imm as u32;
    }
}
#[allow(dead_code)]
pub fn bne(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    if operand1 != operand2
    {
        cpu.regs.program_counter += instruction.imm as u32;
    }
}
#[allow(dead_code)]
pub fn blt(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    if operand1 < operand2
    {
        cpu.regs.program_counter += instruction.imm as u32;
    }
}
#[allow(dead_code)]
pub fn bge(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    if operand1 >= operand2
    {
        cpu.regs.program_counter += instruction.imm as u32;
    }
}
#[allow(dead_code)]
pub fn bltu(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    if operand1 < operand2
    {
        cpu.regs.program_counter += instruction.imm as u32;
    }
}
#[allow(dead_code)]
pub fn bgeu(cpu: &mut Cpu, instruction: &DecodedInst){
    let operand1 = cpu.regs.get(instruction.rs1 as usize);
    let operand2 = cpu.regs.get(instruction.rs2 as usize);
    if operand1 == operand2
    {
        cpu.regs.program_counter += instruction.imm as u32;
    }
}

// TYPE J/I - OPCODE 1101111
#[allow(dead_code)]
pub fn jal(cpu: &mut Cpu, instruction: &DecodedInst){
    cpu.regs.program_counter += instruction.imm as u32;
    cpu.regs.set(instruction.rs1 as usize, cpu.regs.get(instruction.rs1 as usize) + 4);
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

pub struct DecodedInst{
    pub raw_inst:   u32, // Raw instruction.
    pub opcode:     u32, // Opcode field.
    pub rs1:        u32, // Source register 1.
    pub rs2:        u32, // Source register 2.
    pub rd:         u32, // Destination register.
    pub imm:        u32, // Sign extended immediate value for I type.
    pub funct3:     u32, // Sub-function value I type.
    pub funct7:     u32, // Sub-function value R type
}

impl DecodedInst {
    pub fn decode(coded_inst: u32) -> Result<DecodedInst, String> {
        let opcode: u32 = coded_inst & 0b0000000000000000000000001111111;
        if opcode == 0b0110011 {         // R Type
            Ok(decode_r_type(coded_inst))
        }else if opcode == 0b0010011 || opcode == 0b0000011 || opcode == 0b1100111 || opcode == 0b1110011 {  // I Type
            Ok(decode_i_type(coded_inst))
        }else if opcode == 0b0100011 {  // S Type
            Ok(decode_s_type(coded_inst))
        }else if opcode == 0b1100011 {  // B Type
            Ok(decode_b_type(coded_inst))
        }else if opcode == 0b0110111 || opcode == 0b0010111{  // U Type
            Ok(decode_u_type(coded_inst))
        }else if opcode == 0b1101111 {  // J Type
            Ok(decode_j_type(coded_inst))
        }else{
            Err("Instruction could not be decoded".to_string())
        }
    }
    // Function to print the decoded instruction in a human readable way.
    #[allow(dead_code)]
    pub fn to_string(&self) -> String{
        format!("raw_inst: {:#032b}\n\
                    opcode:   {:#032b}\n\
                    rd:       {:#032b}\n\
                    rs1 as usize:      {:#032b}\n\
                    rs2 as usize:      {:#032b}\n\
                    imm:      {:#032b}\n\
                    funct3:   {:#032b}\n\
                    funct7:   {:#032b}", self.raw_inst, self.opcode, self.rd, self.rs1 as usize, self.rs2 as usize, self.imm, self.funct3, self.funct7)
    }
}


fn decode_r_type(coded_inst: u32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     (coded_inst & 0b0000000000000000000111110000000),
        rs1:    (coded_inst & 0b0000000000011111000000000000000),
        rs2:    (coded_inst & 0b0000000111100000000000000000000),
        imm: 0,
        funct3: (coded_inst & 0b0000000000000000111000000000000),
        funct7: (coded_inst & 0b1111111000000000000000000000000)
    }
}

fn decode_i_type(coded_inst: u32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     (coded_inst & 0b0000000000000000000111110000000) >> 7,
        rs1:    (coded_inst & 0b0000000000011111000000000000000) >> 15,
        rs2:    0,
        imm:    (coded_inst & 0b1111111111100000000000000000000) >> 20,
        funct3: (coded_inst & 0b0000000000000000111000000000000) >> 12,
        funct7: 0
    }
}

fn decode_s_type(coded_inst: u32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     0,
        rs1:    (coded_inst & 0b0000000000011111000000000000000),
        rs2:    (coded_inst & 0b0000000111100000000000000000000),
        imm:    (coded_inst & 0b1111111000000000000000000000000) >> 17  | (coded_inst & 0b0000000000000000000111110000000) >> 7,
        funct3: (coded_inst & 0b0000000000000000111000000000000),
        funct7: 0
    }
}

fn decode_b_type(coded_inst: u32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     0,
        rs1:    (coded_inst & 0b0000000000011111000000000000000),
        rs2:    (coded_inst & 0b0000000111100000000000000000000),
        imm:    (coded_inst & 0b1111111000000000000000000000000) | (coded_inst & 0b0000000000000000000111110000000),
        funct3: (coded_inst & 0b0000000000000000111000000000000),
        funct7: 0
    }
}

fn decode_u_type(coded_inst: u32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     (coded_inst & 0b0000000000000000000111110000000),
        rs1:    0,
        rs2:    0,
        imm:    (coded_inst & 0b1111111111111111111000000000000),
        funct3: 0,
        funct7: 0
    }
}

fn decode_j_type(coded_inst: u32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     (coded_inst & 0b0000000000000000000111110000000),
        rs1:    0,
        rs2:    0,
        imm:    (coded_inst & 0b1111111111111111111000000000000),
        funct3: 0,
        funct7: 0
    }
}