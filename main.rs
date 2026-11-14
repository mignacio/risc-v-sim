use std::env;

use crate::cpu::Cpu;
// use instructions::InstFunc;

mod cpu;

fn main(){
    let mut cpu: Cpu;
    
    env::set_var("RUST_BACKTRACE", "1");

    let coded_instruction_arr: [i32; 9] = [0x00730293, 0x00734293, 0x00736293, 0x00737293, 0x00731293, 0x00735293, 0x40735293, 0x00732293, 0x00733293];

}