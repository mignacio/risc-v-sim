#[allow(dead_code)]
pub const REGISTER_LENGTH: i32 = 32;
#[allow(dead_code)]
pub const REGISTER_WIDTH: i32 = 32;

//TODO: Find a way to avoid having to use static mut
static mut REGISTERS: [i32; 32] = [0; 32];

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

pub fn fake_set(_index: i32, _value: i32){

}
