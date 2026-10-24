#[allow(dead_code)]
pub const REGISTER_LENGTH: u32 = 32;
#[allow(dead_code)]
pub const REGISTER_WIDTH: u32 = 32;

//TODO: Find a way to avoid having to use static mut
// Create a struct with IMPLs to read and write from the registers
// Add x0 that ignores writes and is always `0`.
// Make it also hold the program_counter register.

static mut REGISTERS: [u32; 32] = [0; 32];
pub static mut PROGRAM_COUNTER: u32 = 0;

#[allow(dead_code)]
pub fn print(){
    unsafe{
        for reg in REGISTERS{
            println!{"{:#032b}", reg};
        }
        println!("PROGRAM COUNTER: {:#032b}", PROGRAM_COUNTER);
    }
}

pub fn set(index: u32, value: u32){
    unsafe{
        REGISTERS[index as usize] = value;
    }
}

pub fn get(index: u32) -> u32{
    unsafe{
        REGISTERS[index as usize]
    }
}

