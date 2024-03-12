mod instructions;

use instructions::registers;
use instructions::RV32I;
use instructions::InstFunc;

fn main(){

    registers::set(15, 15);
    //registers::print();

    let decoded_inst: RV32I = RV32I::decode(0x00730293);
    println!("{}", decoded_inst.to_string());

    let instruction_function: InstFunc = instructions::OPCODES.get(&decoded_inst.opcode).cloned().expect("REASON");
    instruction_function(decoded_inst);

}