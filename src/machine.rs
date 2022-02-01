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

    #[derive(FromPrimitive, ToPrimitive)]
    #[derive(Debug)]
    #[derive(PartialEq)]
    #[repr(u8)]
    pub enum OpCode{
        Halt, Add, Subtract, And, Xor, LeftShift, RightShift,
        LoadAddress, Load, Store, LoadIndirect, StoreIndirect,
        BranchZero, BranchPositive, JumpRegister, JumpAndLink,
    }


    #[derive(Debug)]
    pub struct ProgramLoadWord {
        address: u8,
        content: u16,
    }

    #[derive(Debug)]
    pub struct Instruction {
        op: u8,
        // really u4 - one hex digit
        d: u8,
        // really u4 - one hex digit
        s: u8,
        // really u4 - one hex digit
        t: u8,
        // really u4 - one hex digit
        address: u8,
    }

    impl Instruction {
        pub fn new(op: u8, d: u8, s: u8, t: u8, address: u8) -> Self {
            Self { op, d, s, t, address }
        }
        pub fn get_values(&self) -> (u8, u8, u8, u8, u8) {
            (self.op, self.d, self.s, self.t, self.address)
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
            for word in loads {
                self.memory[word.address as usize] = word.content
            }
        }
        fn set_program_counter(&mut self, pc: u8) {
            self.pc = pc;
        }
        pub(crate) fn get_next_instruction(&mut self) -> Instruction {
            let local_pc = self.pc;
            let word = self.get_memory_word(local_pc as usize);
            self.set_program_counter(local_pc + 1); // default
            let op= (word >> 12) as u8;
            let d = (word >> 8 & 0xF) as u8;
            let s = (word >> 4 & 0xF) as u8;
            let t = (word & 0xF) as u8;
            let address = (word & 0xFF) as u8;
            let format2 = vec![7 as u8, 8, 9, 0xC, 0xD, 0xF];
            if op == 0 {
                Instruction::new(0, 0, 0, 0, 0)
            } else if op == 0xE {
                Instruction::new(op, d, 0, 0, 0)
            } else {
                if format2.contains(&op) {
                    Instruction::new(op, d, 0, 0, address)
                } else {
                    Instruction::new(op, d, s, t, 0)
                }
            }
        }
        fn execute_next_instruction(&self, instruction: Instruction) -> bool {
            let (op, d, s, t, address) =  instruction.get_values();
            // match op {
            //    1 => {},
            //    2 => {},
            //    3 => {},
            // };
            true
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
                   self.regs[3], self.regs[4]
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
                print!("  {:02X}:", start);
                for loc in start..(start + 16) {
                    print!(" {:04X}", self.memory[loc]);
                }
                println!()
            }
        }
    }

    #[cfg(test)]
    mod machine_tests {
        use super::*;
        use crate::machine::external_env::external_env::ExternalEnv;
        use crate::machine::program_reader::program_reader::ProgramReader;

        mod read_write_memory {
            use super::*;

            fn get_word_from_env(env: &mut ExternalEnv) -> u16 {
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
            ) {
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
                    100, 0x0001, 1
                );
                test_new_word_env_to_mem(
                    &mut env, &mut machine,
                    120, 0x0002, 3
                );
                test_new_word_env_to_mem(
                    &mut env, &mut machine,
                    255, 0x0003, 6
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
            fn test_write_mem_word_to_env() {
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

        fn loaded_machine() -> Machine {
            let test_program_strings = vec![
                "10: 8AFF",   // read to R[A]                  a = StdIn.readInt();
                "11: 8BFF",   // read to R[B]                  b = StdIn.readInt();
                "12: 7C00",   // R[C] <- 0000                  c = 0;
                "13: 7101",   // R[1] <- 0001                  the constant 1
                "14: CA18",   // if (R[A] == 0) goto 18        while (a != 0) {
                "15: 1CCB",   // R[C] <- R[C] + R[B]              c += b;
                "16: 2AA1",   // R[A] <- R[A] - R[1]              a -= 1;
                "17: C014",   // goto 14                       }
                "18: 9CFF",   // write from R[C]               StdOut.println(c);
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
            machine
        }

        mod loaded_program {
            use super::*;

            #[test]
            fn program_load() {
                let machine = loaded_machine();

                let expected = [0x8AFF, 0x8BFF, 0x7C00, 0x7101, 0xCA18,
                    0x1CCB, 0x2AA1, 0xC014, 0x9CFF, 0x0000];
                for i in 0..expected.len() {
                    assert_eq!(machine.get_memory_word(16 + i), expected[i]);
                }
            }
        }
        mod fetch_instruction {
            use super::*;

            #[test]
            fn instruction_from_memory_word() {
                let machine = loaded_machine();
                let word = machine.get_memory_word(16);
                assert_eq!(0x8AFF, word);
                let op: u8 = (word >> 12) as u8;
                assert_eq!(8, op);
                let d = (word >> 8 & 0xF) as u8;
                assert_eq!(0xA, d);
                let s = (word >> 4 & 0xF) as u8;
                assert_eq!(0xF, s);
                let t = (word & 0xF) as u8;
                assert_eq!(0xF, t);
                let address = (word & 0xFF) as u8;
                let format2 = vec![7 as u8, 8, 9, 0xC, 0xD, 0xF];
                let instruction = {
                    if op == 0 {
                        Instruction::new(0, 0, 0, 0, 0)
                    } else if op == 0xE {
                        Instruction::new(op, d, 0, 0, 0)
                    } else {
                        if format2.contains(&op) {
                            Instruction::new(op, d, 0, 0, address)
                        } else {
                            Instruction::new(op, d, s, t, 0)
                        }
                    }
                };
                assert!(format2.contains(&op));
                assert_eq!(op, instruction.op);
                assert_eq!(d, instruction.d);
                assert_eq!(0, instruction.s);
                assert_eq!(0, instruction.t);
                assert_eq!(0xFF, instruction.address);
            }

            #[test]
            fn next_instruction(){
                let mut machine = loaded_machine();
                machine.set_program_counter(0x10);
                assert_eq!(machine.pc, 0x10);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x11);
                assert_eq!(0x8, instruction.op);
                assert_eq!(0xA, instruction.d);
                assert_eq!(0xFF, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x12);
                assert_eq!(0x8, instruction.op);
                assert_eq!(0xB, instruction.d);
                assert_eq!(0xFF, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x13);
                assert_eq!(0x7, instruction.op);
                assert_eq!(0xC, instruction.d);
                assert_eq!(0x00, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x14);
                assert_eq!(0x7, instruction.op);
                assert_eq!(0x1, instruction.d);
                assert_eq!(0x01, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x15);
                assert_eq!(0xC, instruction.op);
                assert_eq!(0xA, instruction.d);
                assert_eq!(0x18, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x16);
                assert_eq!(0x1, instruction.op);
                assert_eq!(0xC, instruction.d);
                assert_eq!(0xC, instruction.s);
                assert_eq!(0xB, instruction.t);
                assert_eq!(0x00, instruction.address);
            }
        }
        mod instruction_execution {
            use super::*;
            use num;

            #[test]
            fn test_from_primitive_to_opcode() {
                let v: [OpCode; 3]= [
                    num::FromPrimitive::from_u8(0).unwrap(),
                    num::FromPrimitive::from_u8(1).unwrap(),
                    num::FromPrimitive::from_u8(2).unwrap(),
                ];

                let expected: [OpCode; 3] =  [
                    OpCode::Halt,
                    OpCode::Add,
                    OpCode::Subtract,
                ];

                for i in 0..3{
                    assert_eq!(v[i], expected[i]);
                }
            }

            #[test]
            fn test_to_primitive_opcode() {
                let v: [Option<u8>; 3] = [
                    num::ToPrimitive::to_u8(&OpCode::Halt),
                    num::ToPrimitive::to_u8(&OpCode::Add),
                    num::ToPrimitive::to_u8(&OpCode::Subtract),
                ];

                assert_eq!(v, [Some(0), Some(1), Some(2)]);
            }

            #[test]
            fn basic_instruction() {  // add, subtract, and, xor, left-shift right-shift
                let mut machine = Machine::new();
                let d: u8 = 1;
                let s: u8 = 2;
                let t: u8 = 3;
                machine.regs[s as usize] = 0x10;
                machine.regs[t as usize] = 0x08;
                let op: u8 = 1; // add: R[d] <- R[s] + R[t]
                let instruction = Instruction::new(op, d, s, t, 0);
                machine.execute_next_instruction(instruction);
                // let op = 2; // subtract: R[d] <- R[s] - R[t]
                // let op = 3; // and: R[s] & R[t]
                // let op = 4; // xor: R[d] <- R[s] ^ R[t]
                // let op = 5; // left shift:	R[d] <- R[s] << R[t]
                // let op = 6; // right shift: R[d] <- R[s] >> R[t]
            }

        }
    }
}