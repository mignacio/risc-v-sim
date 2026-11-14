mod instructions;

use std::env;
// use instructions::InstFunc;

mod cpu;
mod instructions;

fn main(){
    let mut cpu = cpu::Cpu::new();
    
    env::set_var("RUST_BACKTRACE", "1");
    let mut regs = Registers { regs: [0; 32], program_counter: 0 };
    //registers::print();
    regs.print();

    regs.step();
    regs.set(1, 0xAAAAAAAA);
    let reg5 = regs.get(1);
    regs.set(5, reg5);
    regs.clear(1);

    regs.print();
    // let coded_instruction_arr: [i32; 9] = [0x00730293, 0x00734293, 0x00736293, 0x00737293, 0x00731293, 0x00735293, 0x40735293, 0x00732293, 0x00733293];

    // for coded in coded_instruction_arr{
    //     let decoded_inst: DecodedInst;
    //     match DecodedInst::decode(coded) {
    //         Ok(v) => decoded_inst = v,
    //         Err(_e) => break
    //     }
    //     //println!("{}", decoded_inst.to_string());
    //     let decoded_funct: i32 = decoded_inst.funct3;
    //     let instruction_function: InstFunc = instructions::I_CODES.get( &decoded_funct).cloned().expect("REASON");
    //     instruction_function(decoded_inst);
    // }
}