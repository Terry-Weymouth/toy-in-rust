#![allow(dead_code)]
#![allow(unused_variables)]

pub use self::{external_env::*};

mod external_env;

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
        let memory: [u16; 256] = [0; 256];
        let regs: [u16; 16] = [0; 16];
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
    pub(crate) fn get_memory_word(&self, index: usize) -> u16 {
        assert!(index < 256);
        self.memory[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod read_write_memory {
        use super::*;

        fn get_word_from_env(env: &mut ExternalEnv) -> u16{
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
            word
        }

        fn test_new_word_env_to_mem(
            env: &mut ExternalEnv, machine: &mut Machine,
            index: usize, expected_value: u16, expected_sum: u16
        ){
            let word = get_word_from_env(env);
            assert_eq!(expected_value, word);
            machine.set_memory_word(index, word);
            let sum_across_memory: u16 =
                machine.memory.iter().sum();
            assert_eq!(expected_sum, sum_across_memory);
        }

        #[test]
        fn initial_memory_is_zero() {
            let machine = Machine::new();
            let sum_across_memory: u16 =
                machine.memory.iter().sum();
            assert_eq!(0, sum_across_memory);
        }

        // test index out of bounds for size of memory todo!()

        #[test]
        fn read_word_from_env_load_to_memory() {
            let mut machine = Machine::new();
            let mut env = ExternalEnv::new(vec![0x0001, 0x0002, 0x0003]);
            let sum_across_memory: u16 =
                machine.memory.iter().sum();
            assert_eq!(0, sum_across_memory);
            test_new_word_env_to_mem(
                &mut env, &mut machine,
                100,0x0001, 1
            );
            test_new_word_env_to_mem(
                &mut env, &mut machine,
                120,0x0002, 3
            );
            test_new_word_env_to_mem(
                &mut env, &mut machine,
                255,0x0003, 6
            );
            machine.set_memory_word(100, 0);
            let sum_across_memory: u16 =
                machine.memory.iter().sum();
            assert_eq!(5, sum_across_memory);
            machine.set_memory_word(120, 0);
            let sum_across_memory: u16 =
                machine.memory.iter().sum();
            assert_eq!(3, sum_across_memory);
            machine.set_memory_word(255, 0);
            let sum_across_memory: u16 =
                machine.memory.iter().sum();
            assert_eq!(0, sum_across_memory);
        }
        #[test]
        fn test_write_mem_word_to_env(){
            let mut machine = Machine::new();
            let mut env = ExternalEnv::new(vec![0x0001, 0x0002, 0x0003]);
            for index in vec![100, 101, 102] {
                machine.set_memory_word(index, get_word_from_env(&mut env));
                let word = machine.get_memory_word(index);
                env.put_word(word);
                assert_eq!(word, env.peek_at_last_output())
            }
        }
    }
}
