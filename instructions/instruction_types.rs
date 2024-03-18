pub struct DecodedInst{
    pub raw_inst:   i32, // Raw instruction.
    pub opcode:     i32, // Opcode field.
    pub rs1:        i32, // Source register 1.
    pub rs2:        i32, // Source register 2.
    pub rd:         i32, // Destination register.
    pub imm:        i32, // Sign extended immediate value for I type.
    pub funct3:     i32, // Sub-function value I type.
    pub funct7:     i32, // Sub-function value R type
}

impl DecodedInst {
    pub fn decode(coded_inst: i32) -> Result<DecodedInst, String> {
        let opcode: i32 = coded_inst & 0b0000000000000000000000001111111;
        if opcode == 0b0110011 {         // R Type
            Ok(decode_r_type(coded_inst))
        }else if opcode == 0b0010011 || opcode == 0b0000011 || opcode == 0b1100111 || opcode == 0b1110011 {  // I Type
            Ok(decode_i_type(coded_inst))
        }else if opcode == 0b0100011 {  // S Type
            Ok(decode_s_type(coded_inst))
        }else if opcode == 0b1100011 {  // B Type
            Ok(decode_b_type(coded_inst))
        }else if opcode == 0b0110111 || opcode == 0b0010111{  // U Type
            Ok(decode_u_type(coded_inst))
        }else if opcode == 0b1101111 {  // J Type
            Ok(decode_j_type(coded_inst))
        }else{
            Err("Instruction could not be decoded".to_string())
        }
    }
    // Function to print the decoded instruction in a human readable way.
    #[allow(dead_code)]
    pub fn to_string(&self) -> String{
        format!("raw_inst: {:#032b}\n\
                    opcode:   {:#032b}\n\
                    rd:       {:#032b}\n\
                    rs1:      {:#032b}\n\
                    rs2:      {:#032b}\n\
                    imm:      {:#032b}\n\
                    funct3:   {:#032b}\n\
                    funct7:   {:#032b}", self.raw_inst, self.opcode, self.rd, self.rs1, self.rs2, self.imm, self.funct3, self.funct7)
    }
}


fn decode_r_type(coded_inst: i32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     (coded_inst & 0b0000000000000000000111110000000),
        rs1:    (coded_inst & 0b0000000000011111000000000000000),
        rs2:    (coded_inst & 0b0000000111100000000000000000000),
        imm: 0,
        funct3: (coded_inst & 0b0000000000000000111000000000000),
        funct7: (coded_inst & 0b1111111000000000000000000000000)
    }
}

fn decode_i_type(coded_inst: i32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     (coded_inst & 0b0000000000000000000111110000000) >> 7,
        rs1:    (coded_inst & 0b0000000000011111000000000000000) >> 15,
        rs2:    0,
        imm:    (coded_inst & 0b1111111111100000000000000000000) >> 20,
        funct3: (coded_inst & 0b0000000000000000111000000000000) >> 12,
        funct7: 0
    }
}

fn decode_s_type(coded_inst: i32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     0,
        rs1:    (coded_inst & 0b0000000000011111000000000000000),
        rs2:    (coded_inst & 0b0000000111100000000000000000000),
        imm:    (coded_inst & 0b1111111000000000000000000000000) >> 17  | (coded_inst & 0b0000000000000000000111110000000) >> 7,
        funct3: (coded_inst & 0b0000000000000000111000000000000),
        funct7: 0
    }
}

fn decode_b_type(coded_inst: i32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     0,
        rs1:    (coded_inst & 0b0000000000011111000000000000000),
        rs2:    (coded_inst & 0b0000000111100000000000000000000),
        imm:    (coded_inst & 0b1111111000000000000000000000000) | (coded_inst & 0b0000000000000000000111110000000),
        funct3: (coded_inst & 0b0000000000000000111000000000000),
        funct7: 0
    }
}

fn decode_u_type(coded_inst: i32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     (coded_inst & 0b0000000000000000000111110000000),
        rs1:    0,
        rs2:    0,
        imm:    (coded_inst & 0b1111111111111111111000000000000),
        funct3: 0,
        funct7: 0
    }
}

fn decode_j_type(coded_inst: i32) -> DecodedInst{
    DecodedInst{
        raw_inst: coded_inst,
        opcode: (coded_inst & 0b0000000000000000000000001111111),
        rd:     (coded_inst & 0b0000000000000000000111110000000),
        rs1:    0,
        rs2:    0,
        imm:    (coded_inst & 0b1111111111111111111000000000000),
        funct3: 0,
        funct7: 0
    }
}