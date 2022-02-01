#![allow(dead_code)]
#![allow(unused_variables)]

pub mod external_env;
pub mod program_reader;

pub mod machine {
    #[derive(Debug)]
    pub struct Machine {
        pc: u8,
        regs: [u16; 16],
        pub(crate) memory: [u16; 256],
    }

    #[derive(Debug)]
    pub struct ProgramLoadWord {
        address: u8,
        content: u16,
    }

    #[derive(Debug)]
    pub struct Instruction {
        op: u8,        // really u4 - one hex digit
        d: u8,         // really u4 - one hex digit
        s: u8,         // really u4 - one hex digit
        t: u8,         // really u4 - one hex digit
        address: u8,
    }

    impl Instruction {
        pub fn new(op: u8, d: u8, s: u8, t: u8, address: u8) -> Self {
            Self { op, d, s, t, address }
        }
    }

    impl ProgramLoadWord {
        pub fn new(address: u8, content: u16) -> Self {
            Self {
                address,
                content,
            }
        }
    }

    impl Machine {
        pub(crate) fn new() -> Self {
            let pc: u8 = 0;
            let memory: [u16; 256] = [0; 256];
            let regs: [u16; 16] = [0; 16];
            Self {
                pc,
                regs,
                memory,
            }
        }
        pub fn load(&mut self, loads: Vec<ProgramLoadWord>) {
            for word in loads{
                self.memory[word.address as usize] = word.content
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
        pub fn dump(&self) {
            print!("pc: {:2x} regs: 0={:2x}, 1={:2x}, 2={:2x}, 3={:2x}, 4={:2x},",
                     self.pc, self.regs[0], self.regs[1], self.regs[2],
                     self.regs[3] ,self.regs[4]
            );
            print!(" 5={:2x}, 6={:2x}, 7={:2x}, 8={:2x}, 9={:2x}, A={:2x},",
                     self.regs[5], self.regs[6], self.regs[7],
                     self.regs[8], self.regs[9], self.regs[10],
            );
            println!(" B={:2x}, C={:2x}, D={:2x}, E={:2x}, F={:2x}",
                     self.regs[11], self.regs[12], self.regs[13],
                     self.regs[14], self.regs[15],
            );
            println!(" memory...");
            for i in 0..15 {
                let start = 16 * i;
                print! ("  {:02X}:", start);
                for loc in start..(start + 16){
                    print!(" {:04X}", self.memory[loc]);
                }
                println!()
            }
        }
    }
}

#[cfg(test)]
mod machine_tests {
    use crate::machine::external_env::external_env::ExternalEnv;
    use crate::machine::machine::Machine;
    use crate::machine::program_reader::program_reader::ProgramReader;

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
    mod load_program{
        use super::*;

        #[test]
        fn program_load()
        {
            let test_program_strings = vec![
                "10: 8AFF",   // read R[A]                     a = StdIn.readInt();
                "11: 8BFF",   // read R[B]                     b = StdIn.readInt();
                "12: 7C00",   // R[C] <- 0000                  c = 0;
                "13: 7101",   // R[1] <- 0001                  the constant 1
                "14: CA18",   // if (R[A] == 0) goto 18        while (a != 0) {
                "15: 1CCB",   // R[C] <- R[C] + R[B]              c += b;
                "16: 2AA1",   // R[A] <- R[A] - R[1]              a -= 1;
                "17: C014",   // goto 14                       }
                "18: 9CFF",   //write R[C]                    StdOut.println(c);
                "19: 0000"    // halt
            ];
            let mut reader = ProgramReader::new();
            let mut program_text: Vec<String> = vec![];
            for s in test_program_strings {
                program_text.push(String::from(s));
            }
            reader.load_from_vec(program_text);
            let loads = reader.parse();
            let mut machine = Machine::new();
            machine.load(loads);

            let expected = [0x8AFF, 0x8BFF, 0x7C00, 0x7101, 0xCA18,
                0x1CCB, 0x2AA1, 0xC014, 0x9CFF, 0x0000];
            for i in 0..expected.len() {
                assert_eq!(machine.get_memory_word(16 + i), expected[i]);
            }
        }
    }
}
