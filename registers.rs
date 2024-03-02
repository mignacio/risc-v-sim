pub const REGISTER_LENGTH: i32 = 32;
pub const REGISTER_WIDTH: i32 = 32;

static mut REGISTERS: [i32; 32] = [0; 32];

static mut PROGRAM_COUNTER: i32 = 0;

pub fn print(){
    unsafe{
        for reg in REGISTERS{
            println!{"{:#032b}", reg};
        }
        println!("PROGRAM COUNTER: {:#032b}", PROGRAM_COUNTER);
    }
}

pub fn set(index: usize, value: i32){
    unsafe{
        REGISTERS[index] = value;
    }
}

pub fn get(index: usize) -> i32{
    unsafe{
        REGISTERS[index]
    }
}