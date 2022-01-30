#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

extern crate core;

#[derive(Debug)]
pub struct ExternalEnv {
    input: Vec<u16>,
}

impl ExternalEnv{
    fn new(input: Vec<u16>) -> Self {
        Self {
            input
        }
    }
    fn get_next_word(&mut self) -> Option<u16> {
        if self.input.len() == 0 {
            return None
        }
        let value = self.input.remove(0);
        Option::from(value)
    }
}

#[derive(Debug)]
pub struct Machine {
    pc: u8,
    regs: [u16; 16],
    memory: [u16; 256],
}

#[derive(Debug)]
pub struct Instruction {
    op:    u8, // really u4 - one hex digit
    d:     u8, // really u4 - one hex digit
    s:     u8, // really u4 - one hex digit
    t:     u8, // really u4 - one hex digit
    address: u8,
}

impl Machine {
    fn new() -> Self {
        let pc: u8 = 0;
        let mut memory: [u16; 256] = [0; 256];
        let mut regs: [u16; 16] = [0; 16];
        Self {
            pc,
            regs,
            memory,
        }
    }
    fn set_program_counter(&mut self, pc: u8) {
        self.pc = pc;
    }
    fn execute_next_instruction(&self, instruction: &Instruction) -> bool {
        if instruction.op == 0 {
            return false;
        }

        let reg_ref_t_content_read = self.regs[instruction.t as usize];
        if ((instruction.op == 8) && (instruction.address == 255)) ||
            (instruction.op == 10 && reg_ref_t_content_read == 255u16) {
            //&self.read_word_into_memory_255();
        }

        self.alu_operation(instruction);

        let reg_ref_t_content_write = self.regs[instruction.t as usize];
        if (instruction.op == 9 && instruction.address == 255) ||
            (instruction.op == 11 && reg_ref_t_content_write == 255u16) {
            //self.write_word_from_memory_255();
        }

        //&self.regs[0] = &0u16;
        let trimmed_d_reg_value: u16 = &self.regs[instruction.d as usize] & 0xFFFF;
        //&self.regs[instruction.d as usize] = &trimmed_d_reg_value;

        true
    }
    fn alu_operation(&self, _instruction: &Instruction) {
        todo!()
    }
    pub(crate) fn set_memory_word(&mut self, index: usize, value: u16) {
        assert!(index < 256);
        self.memory[index] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod read_write_memory_255 {
        use super::*;

        fn test_next_word(env: &mut ExternalEnv, expected: u16) {
            let opt_value = env.get_next_word();
            let mut word: u16 = 0;
            match opt_value
            {
                Some(value) => {
                    word = value;
                },
                None => {
                    assert!(false)
                }
            }
            assert_eq!(expected, word);
        }

        #[test]
        fn initial_memory_is_zero() {
            let machine = Machine::new();
            let sum_across_memory: u16 =
                machine.memory.iter().sum();
            assert_eq!(0, sum_across_memory);
        }

        #[test]
        fn read_from_env() {
            let mut env = ExternalEnv::new(vec![0x1234, 0x2345, 0x3456]);
            test_next_word(&mut env, 0x1234);
            test_next_word(&mut env, 0x2345);
            test_next_word(&mut env, 0x3456);
        }

        #[test]
        fn write_word_from_env_to_memory() {
            let mut machine = Machine::new();
            let mut env = ExternalEnv::new(vec![0x0001, 0x0002, 0x0003]);
            let sum_across_memory: u16 =
                machine.memory.iter().sum();
            assert_eq!(0, sum_across_memory);
            let opt_value = env.get_next_word();
            let mut word: u16 = 0;
            match opt_value
            {
                Some(value) => {
                    word = value;
                },
                None => {
                    assert!(false)
                }
            }
            let expected:u16 = 0x0001;
            assert_eq!(expected, word);
            let index = 255;
            machine.set_memory_word(index, word);
            let sum_across_memory: u16 =
                machine.memory.iter().sum();
            assert_eq!(1, sum_across_memory);
        }
    }

    mod read_from_extern_load_memory {

        struct MemoryHolder {
            memory: [u16; 256],
        }
        impl MemoryHolder {
            fn new() -> Self {
                let memory: [u16; 256] = [0; 256];
                Self {
                    memory
                }
            }
            fn set_memory_to_value(&mut self, index: usize, value: u16) {
                assert!(index < 256);
                self.memory[index] = value;
            }
        }

        struct ExternalSource{
            input: Vec<u16>,
        }
        impl ExternalSource{
            fn new(input: Vec<u16>) -> Self {
                Self {
                    input
                }
            }
            fn get_next_word(&mut self) -> Option<u16> {
                if self.input.len() == 0 {
                    return None
                }
                let value = self.input.remove(0);
                Option::from(value)
            }
        }

        #[test]
        fn test_load_memory_from_external() {
            let mut machine = MemoryHolder::new();
            let mut env = ExternalSource::new(vec![0x1234, 0x2345, 0x3456]);
            let index = 5;
            let opt_value = env.get_next_word();
            match opt_value
            {
                Some(value) => {
                    assert_eq!(0, machine.memory[index]);
                    machine.set_memory_to_value(index, value);
                    assert_eq!(0x1234, machine.memory[index]);
                },
                None =>{
                    assert!(false)
                }
            }
        }
    }
}
