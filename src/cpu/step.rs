use super::{instructions::{InstParam, Instructions}, registers::{Register16Bit, Register8Bit}, CPU};

impl CPU {
    // Gets a 8-bit value from the HL register
    fn get_n8_from_hl(&self) -> u8 {
        self.memory.read_byte(self.get_16bit_register(Register16Bit::HL))
    }

    /// Does a step (calls function and sets last_step_result), 
    /// ensure to first set the next instruction
    /// by decoding it (see `decode.rs`)
    pub fn step(&mut self) {
        self.last_step_result = match &self.next_instruction {
            Instructions::NOP => self.nop(),
            Instructions::ADD(param) => {
                match param {
                    InstParam::Register8Bit(register) => self.add_a_r8(register.clone()),
                    InstParam::Number8Bit(number) => self.add_a_hl(),
                    _ => panic!("ADD with {:?} not implemented", param),
                }
            }
            Instructions::LD(target, source) => {
                match target {
                    InstParam::Register8Bit(target_register) => {
                        if *target_register == Register8Bit::A {
                            match source {
                                InstParam::Register16Bit(source_register) => self.ld_a_r16( *source_register),
                                InstParam::Number16Bit(source_number) => self.ld_a_n16( *source_number),
                                InstParam::Register8Bit(source_register) => self.ld_r8_r8(*target_register, *source_register),
                                _ => panic!("Handling of {:?} not implemented", source),
                            }
                        } else {
                            match source {
                                InstParam::Register8Bit(source_register) => self.ld_r8_r8(*target_register, *source_register),
                                InstParam::Number8Bit(source_number) => self.ld_r8_n8(*target_register, *source_number),
                                InstParam::Register16Bit(source_register) => self.ld_r8_hl(*target_register),
                                _ => panic!("Handling of {:?} not implemented", source),
                            }
                        }
                    },
                    InstParam::Register16Bit(target_register) => {
                        if *target_register == Register16Bit::HL{
                            //TODO what about HLI und HLD?
                            match source {
                                InstParam::Register8Bit(source_register) => self.ld_hl_r8(*source_register),
                                InstParam::Number8Bit(source_number) => self.ld_hl_n8(*source_number),
                                _ => panic!("Handling of {:?} not implemented", source),
                            }
                        } else {
                            match source {
                                InstParam::Number16Bit(source_number) => self.ld_r16_n16(*target_register,*source_number),
                                InstParam::Register8Bit(source_register) => self.ld_r16_a(*target_register),
                                _ => panic!("Handling of {:?} not implemented", source),
                            }
                        }
                    },
                    InstParam::Number16Bit(number) => self.ld_n16_a(*number),
                    _ => panic!("Handling of {:?} not implemented", target),
                }
            }
            _ => panic!("Handling of {:?} not implemented", self.next_instruction),
        }
    }
}