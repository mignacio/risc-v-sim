mod instructions;

use std::env;

use instructions::registers;
use instructions::instruction_types::DecodedInst;
use instructions::InstFunc;

fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    registers::set(15, 15);
    //registers::print();

    let coded_instruction_arr: [i32; 9] = [0x00730293, 0x00734293, 0x00736293, 0x00737293, 0x00731293, 0x00735293, 0x40735293, 0x00732293, 0x00733293];

    for coded in coded_instruction_arr{
        let decoded_inst: DecodedInst;
        match DecodedInst::decode(coded) {
            Ok(v) => decoded_inst = v,
            Err(_e) => break
        }
        //println!("{}", decoded_inst.to_string());
        let decoded_funct: i32 = decoded_inst.funct3;
        let instruction_function: InstFunc = instructions::I_CODES.get( &decoded_funct).cloned().expect("REASON");
        instruction_function(decoded_inst);
    }
}