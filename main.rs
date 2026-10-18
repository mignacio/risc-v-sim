mod instructions;

use std::env;
// use instructions::instruction_types::DecodedInst;
// use instructions::InstFunc;

pub struct Registers {
    regs: [u32; 32],
    program_counter: u32,
}

impl Registers {
    fn print(&self) {
        for reg in self.regs{
            println!{"{:#08X}", reg};
        }
        println!("PC: {:#08X}", self.program_counter);
    }

    fn set(&mut self, index: usize, val: u32) {
        // Register x0 is always 0, cannot be set to a different value.
        if index != 0 {
            self.regs[index] = val;
        }
    }

    fn get(&self, index:usize) -> u32 {
        self.regs[index]
    }

    fn clear(&mut self, index: usize) {
        self.regs[index] = 0;
    }

    fn step(&mut self) {
        self.program_counter+=1;
    }
}

pub struct Cpu {
    regs: Registers,
    memory: Vec<u32>,
}

pub enum CpuFault {
    MisalignedAddress,
    AddressOutOfBounds,
    DecodeError,
    ExecuteError,
}


const MEM_SIZE: u32 = 0xFFFFFFFF; //Placeholder for now
impl Cpu {
    pub fn reset(&mut self) {

    }

    pub fn step(&mut self) -> Result<u32, CpuFault> {
        let fetch_res: u32 = self.fetch(self.regs.program_counter)?;
        Ok(fetch_res)
        // self.decode();
        // self.execute();
        // self.regs.step();
    }

    fn fetch(&mut self, program_counter: u32) -> Result<u32, CpuFault> {
        // Is PC byte aligned?
        if program_counter % 4 != 0 {
            Err(CpuFault::MisalignedAddress)
        }
        // Is it inside memory boundaries?
        else if program_counter > MEM_SIZE {
            Err(CpuFault::AddressOutOfBounds)
        }
        else {
            let res:Option<&u32> = self.memory.get(program_counter as usize);
            match res {
                Some(value) => Ok(*value),
                None => Ok(0),
            }
        }
    }

    fn decode(&mut self) {

    }

    fn execute(&mut self) {

    }
}

fn main(){
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