#[allow(dead_code)]
pub const REGISTER_LENGTH: i32 = 32;
#[allow(dead_code)]
pub const REGISTER_WIDTH: i32 = 32;

pub const PC_INDEX: i32 = 31;

//TODO: Find a way to avoid having to use static mut
static mut REGISTERS: [i32; 32] = [0; 32];

pub static mut PROGRAM_COUNTER: u32 = 0;

#[allow(dead_code)]
pub fn print() -> String{
    let mut return_string: String = String::new();
    unsafe{
        for (i, reg) in REGISTERS.iter().enumerate(){
            return_string.push_str(&format!("x{:>2}: {:#032b}\n", i, reg));
        }
        return_string.push_str(&format!(" PC: {:#032b}", PROGRAM_COUNTER));
    }
    return return_string;
}

pub fn set(index: i32, value: i32){
    unsafe{
        REGISTERS[index as usize] = value;
    }
}

pub fn get(index: i32) -> i32{
    unsafe{
        REGISTERS[index as usize]
    }
}

pub fn reset(){
    unsafe{
        for reg in REGISTERS.iter_mut(){
            *reg = 0;
        }
    }
}

