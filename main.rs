mod instructions;

use instructions::registers;
use instructions::RV32I;

fn main(){
    //println!("LENGTH {} WIDTH {}", registers::REGISTER_LENGTH, registers::REGISTER_WIDTH);

    registers::set(15, 15);
    // registers::print();

    //instructions::decoder(0x00730293);
    let decoded_inst: RV32I = RV32I::decode(0x00730293);
    println!("{}", decoded_inst.to_string());

}