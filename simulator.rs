use std::io::Result;

use instructions::registers;
use instructions::instruction_types::DecodedInst;
use instructions::InstFunc;

pub mod instructions;

const CODED_INSTRUCTION_ARR: [i32; 9] = [0x00730293, 0x00734293, 0x00736293, 0x00737293, 0x00731293, 0x00735293, 0x40735293, 0x00732293, 0x00733293];

pub fn step() -> Result<bool>{
    // Fetch the next instruction
    let fetched_instruction = CODED_INSTRUCTION_ARR[registers::get(registers::PC_INDEX) as usize];
    
    // Decode it
    let decoded_inst: DecodedInst;
    match DecodedInst::decode(fetched_instruction) {
        Ok(v) => decoded_inst = v,
        Err(_e) => return Ok(false)
    }
    let decoded_funct: i32 = decoded_inst.funct3;
    let instruction_function: InstFunc = instructions::I_CODES.get(&decoded_funct).cloned().expect("REASON");
    
    // Execute it
    instruction_function(decoded_inst);
    
    // Update program counter
    registers::set(registers::PC_INDEX, registers::get(registers::PC_INDEX) + 1);
    return Ok(true);
}

pub fn reset(){
    registers::reset();
}