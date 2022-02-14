#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate num_derive;

pub mod external_env;
pub mod program_reader;

pub mod machine {
    use super::external_env::external_env::ExternalEnv;

    #[derive(Debug)]
    pub struct Machine {
        pc: u8,
        regs: [u16; 16],
        pub(crate) memory: [u16; 256],
        running: bool,
    }

    #[derive(FromPrimitive, ToPrimitive)]
    #[derive(Debug)]
    #[derive(PartialEq)]
    #[repr(u8)]
    pub enum OpCode{
        Halt, Add, Subtract, And, Xor, ShiftLeft, ShiftRight,
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
        op: OpCode,
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
        pub fn new(op_code_num: u8, d: u8, s: u8, t: u8, address: u8) -> Self {
            let op: OpCode = num::FromPrimitive::from_u8(op_code_num).unwrap();
            Self { op, d, s, t, address }
        }
        pub fn get_values(&self) -> (&OpCode, u8, u8, u8, u8) {
            (&self.op, self.d, self.s, self.t, self.address)
        }
        pub fn is_read_to_memory(&self, regs: &[u16; 16]) -> bool{
            // op code is Load and address == 255
            // or op code is LoadDirect and R[t] == 255
            match self.op {
                OpCode::Load => {self.address == 0xFF}
                OpCode::LoadIndirect => {regs[self.t as usize] == 0xFF}
                _ => false
            }
        }
        pub fn is_write_from_memory(&self, regs: &[u16; 16]) -> bool{
            match self.op {
                OpCode::Store => {self.address == 0xFF}
                OpCode::StoreIndirect => {regs[self.t as usize] == 0xFF}
                _ => false
            }
        }
        pub fn format_for_pp(&self, regs: &[u16; 16], memory: &[u16; 256]) -> String {
            let (op, d, s, t, addr) = self.get_values();
            let dc = regs[d as usize];
            let sc = regs[s as usize];
            let tc = regs[t as usize];
            match self.op {
                // R[d] <- R[s] <op> R[t]
                OpCode::Add | OpCode::Subtract | OpCode::And | OpCode::Xor |
                OpCode::ShiftLeft | OpCode::ShiftRight
                => {
                    format!(
                        "Op: {:?} - d:R[{:01X}] set from: s:R[{:01X}]={}({:04X}) *op* t:R[{:01X}]={}({:04X})",
                        op, d, s, sc, sc, t, tc, tc)
                },
                // R[d] <- addr
                OpCode::LoadAddress => {
                    format!(
                        "Op: {:?} - d:R[{:01X}] becomes {:04X}",
                        op, d, addr )
                },
                // R[d] <- mem[addr]
                OpCode::Load => {
                    if addr == 0xFF {
                        format!(
                            "Op: {:?} - input to d:R[{:01X}] via mem[{:02X}]",
                            op, d, addr
                        )
                    } else {
                        format!(
                            "Op: {:?} - d:R[{:01X}] set from mem[{:02X}]={}({:04X})",
                            op, d, addr, memory[addr as usize], memory[addr as usize])
                    }
                },
                // mem[addr] <- R[d]
                OpCode::Store => {
                    if addr == 0xFF {
                        format!(
                            "Op: {:?} - output from d:R[{:01X}] via mem[{:02X}]",
                            op, d, addr
                        )
                    } else {
                        format!(
                            "Op: {:?} - mem[{:02X}] set from d:R[{:01X}]{}({:04X})",
                            op, addr, d, dc, dc )
                    }
                },
                // R[d] <- mem[R[t]]
                OpCode::LoadIndirect => {
                    if tc == 255 {
                        format!(
                            "Op: {:?} - input to d:R[{:01X}] via mem[{:02X}] as indicated by t:R[{:01X}]",
                            op, d, tc, t )
                    } else {
                        format!(
                            "Op: {:?} - d:R[{:01X}] set from mem[t:R{:01X}]={}({:04X}) where t:R[{:01X}]={:04X}",
                            op, d, t, memory[tc as usize], memory[tc as usize], t, tc)
                    }
                },
                // mem[R[t]] <- R[d]
                OpCode::StoreIndirect => {
                    if tc == 255 {
                        format!(
                            "Op: {:?} - output from d:R[{:01X}] via mem[{:02X}] as indicated by t:R[{:01X}]",
                            op, d, tc, t )
                    } else {
                        format!(
                            "Op: {:?} - mem[t:R[{:01X}]={:04X}] set from d:R[{:01X}]{}({:04X})",
                            op, t, tc, d, dc, dc)
                        }
                },
                OpCode::BranchZero => {
                    format!("Op: {:?} - pc becomes {:02X} when d:R[{:01X}]={:?}({:04X}) == 0",
                             op, addr & 0xFF, d, dc, dc )
                },
                OpCode::BranchPositive => {
                    format!("Op: {:?} - pc becomes {:02X} when d:R[{:01X}]={:?}({:04X}) > 0",
                             op, addr & 0xFF, d, dc, dc )
                },
                OpCode::JumpRegister => {
                    format!("Op: {:?} - pc becomes {:02X} from d:R[{:01X}]={:04X}",
                             op, dc & 0xFF, d, dc )
                },
                //R[d] <- pc; pc <- addr
                OpCode::JumpAndLink => {
                    format!("Op: {:?} - d:R[{:01X}] becomes pc and pc becomes {:02X}",
                             op, d, addr & 0xFF )
                },
                OpCode::Halt => { format!("Op: {:?}", op )},
            }

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
        pub fn new() -> Self {
            let pc: u8 = 0;
            let memory: [u16; 256] = [0; 256];
            let regs: [u16; 16] = [0; 16];
            let running = false;
            Self {
                pc,
                regs,
                memory,
                running,
            }
        }
        pub fn load(&mut self, loads: Vec<ProgramLoadWord>) {
            for word in loads {
                self.memory[word.address as usize] = word.content
            }
        }

        pub fn set_running(&mut self) { self.running = true; }
        pub fn reset_running(&mut self) { self.running = false; }
        pub fn get_running(&self) -> bool { self.running }
        pub fn set_program_counter(&mut self, pc: u8) {
            self.pc = pc;
        }
        pub fn get_program_counter(&self) -> u8 {
            self.pc
        }
        pub fn get_regs(&self) -> Vec<u16> {
            self.regs.to_vec()
        }
        pub fn set_reg(&mut self, i: usize, value: u16) {
            self.regs[i] = value;
        }
        pub fn get_memory(&self) -> Vec<u16> {
            self.memory.to_vec()
        }
        pub fn set_memory_word(&mut self, index: usize, value: u16) {
            assert!(index < 256);
            self.memory[index] = value;
        }
        pub fn get_memory_word(&self, index: usize) -> u16 {
            assert!(index < 256);
            self.memory[index]
        }

        pub(crate) fn get_next_instruction(&mut self) -> Instruction {
            let local_pc = self.pc;
            let word = self.get_memory_word(local_pc as usize);
            self.set_program_counter(local_pc + 1); // default
            self.instruction_from_word(word)
        }
        fn instruction_from_word(&self, word: u16) -> Instruction {
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
        fn execute_next_instruction(&mut self, instruction: &Instruction) -> bool {
            let (op, d, s, t, address) =  instruction.get_values();
            let d = d as usize;
            let s = s as usize;
            let t = t as usize;
            match op {
                // 0	halt	-	exit
                OpCode::Halt => { return false; }
                // 1	add	1	R[d] <- R[s] + R[t]
                OpCode::Add => {self.regs[d] = self.regs[s] + self.regs[t]},
                // 2	subtract	1	R[d] <- R[s] - R[t]
                OpCode::Subtract => {
                    self.regs[d] =
                        {let a: i16 = self.regs[s] as i16 - self.regs[t] as i16; a} as u16
                },
                // 3	and	1	R[d] <- R[s] & R[t]
                OpCode::And => {self.regs[d] = self.regs[s] & self.regs[t]},
                // 4	xor	1	R[d] <- R[s] ^ R[t]
                OpCode::Xor => {self.regs[d] = self.regs[s] ^ self.regs[t]},
                // 5	left shift	1	R[d] <- R[s] << R[t]
                OpCode::ShiftLeft => {self.regs[d] = self.regs[s] << self.regs[t]},
                // 6	right shift	1	R[d] <- R[s] >> R[t]
                OpCode::ShiftRight => {self.regs[d] = self.regs[s] >> self.regs[t]},
                // 7	load address	2	R[d] <- addr
                OpCode::LoadAddress => {self.regs[d] = address as u16},
                // 8	load	2	R[d] <- mem[addr]
                // Note: addr == 255 is special case to load sysin into mem[255]
                //   before execution this instruction; handled in execution loop
                OpCode::Load => {self.regs[d] = self.memory[address as usize]},
                // 9	store	2	mem[addr] <- R[d]
                // Note: addr == 255 is special case to write sysout from mem[255]
                //   after execution this instruction; handled in execution loop
                OpCode::Store => {self.memory[address as usize] = self.regs[d]},
                // A	load indirect	1	R[d] <- mem[R[t]]
                // Note: R[t] == 255 is special case to load sysin into mem[255]
                //   before execution this instruction; handled in execution loop
                OpCode::LoadIndirect => {self.regs[d] = self.memory[self.regs[t] as usize]},
                // B	store indirect	1	mem[R[t]] <- R[d]
                // Note: R[T] == 255 is special case to write sysout from mem[255]
                //   after execution this instruction; handled in execution loop
                OpCode::StoreIndirect => {self.memory[self.regs[t] as usize] = self.regs[d] },
                // C	branch zero	2	if (R[d] == 0) pc <- addr
                OpCode::BranchZero => {if self.regs[d] == 0 {self.pc = address}},
                // D	branch positive	2	if (R[d] > 0) pc <- addr
                OpCode::BranchPositive => {if self.regs[d] > 0 {self.pc = address}},
                // E	jump register	-	pc <- R[d]
                OpCode::JumpRegister => {self.pc = (self.regs[d] & 0xFF) as u8},
                // F	jump and link	2	R[d] <- pc; pc <- addr
                OpCode::JumpAndLink => {self.regs[d] = self.pc as u16; self.pc = address },
            };
            self.regs[0] = 0;                // ensure reg[0] is always 0
            // don't let reg[d] overflow a 16-bit integer
            self.regs[d as usize] = self.regs[d as usize] & 0xFFFF;
            self.pc = self.pc & 0xFF;        // don't let pc overflow an 8-bit integer
            true
        }
        pub fn current_instruction_pp(&self, word: u16) -> String {
            let instruction = self.instruction_from_word(word);
            instruction.format_for_pp(&self.regs, &self.memory)
        }
        pub fn dump_regs(&self) {
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
        }
        pub fn dump_memory(&self) {
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
        pub fn run(&mut self, env: &mut ExternalEnv) {
            self.set_program_counter(0x10);
            self.set_running();
            while self.get_running(){
                self.run_one_step(env, true);
            }
        }

        pub fn run_one_step(&mut self, env: &mut ExternalEnv, print_trace: bool){
            if !self.get_running() {
                return
            }
            let instruction = &self.get_next_instruction();
            if instruction.is_read_to_memory(&self.regs) {
                let option = env.get_next_word();
                let word = option.unwrap();
                self.set_memory_word(0xFF as usize, word);
                if print_trace {
                    println!("Read word to mem[255]: {}({:04X}x)", word, word);
                }
            }
            if print_trace {
                println!(
                    "Execute @ pc = {:02X}x; {}",
                    &self.pc - 1,
                    instruction.format_for_pp(&self.regs, &self.memory)
                );
            }
            let running = self.execute_next_instruction(instruction);
            if running {
                if instruction.is_write_from_memory(&self.regs) {
                    let word = self.get_memory_word(0xFF as usize);
                    env.put_word(word);
                    if print_trace {
                        println!("Write word from mem[255]: {}({:04X}x)", word, word);
                    }
                }
            }
            if running {
                self.set_running();
            } else {
                self.reset_running();
            }
        }
    }

    #[cfg(test)]
    mod machine_tests {
        use super::*;
        use crate::external_env::external_env::ExternalEnv;
        use crate::program_reader::program_reader::ProgramReader;

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
        mod pp_instruction {
            use super::*;

            #[test]
            fn instruction_pp_string() {
                let mut machine = loaded_machine();
                machine.regs[2] = 10;
                machine.regs[3] = 20;
                let instruction = Instruction::new(0x0,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::Halt);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),"Op: Halt");
                let instruction = Instruction::new(0x1,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::Add);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: Add - d:R[1] set from: s:R[2]=10(000A) <op> t:R[3]=20(0014)");
                let instruction = Instruction::new(0x2,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::Subtract);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: Subtract - d:R[1] set from: s:R[2]=10(000A) <op> t:R[3]=20(0014)");
                let instruction = Instruction::new(0x3,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::And);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: And - d:R[1] set from: s:R[2]=10(000A) <op> t:R[3]=20(0014)");
                let instruction = Instruction::new(0x4,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::Xor);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: Xor - d:R[1] set from: s:R[2]=10(000A) <op> t:R[3]=20(0014)");
                let instruction = Instruction::new(0x5,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::ShiftLeft);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: ShiftLeft - d:R[1] set from: s:R[2]=10(000A) <op> t:R[3]=20(0014)");
                let instruction = Instruction::new(0x6,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::ShiftRight);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: ShiftRight - d:R[1] set from: s:R[2]=10(000A) <op> t:R[3]=20(0014)");
                let instruction = Instruction::new(0x7,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::LoadAddress);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: LoadAddress - d:R[1] becomes 0014");
                let instruction = Instruction::new(0x8,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::Load);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: Load - d:R[1] set from mem[14]=51736(CA18)");
                machine.regs[1] = 0x1212;
                let instruction = Instruction::new(0x9,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::Store);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: Store - mem[14] set from d:R[1]4626(1212)");
                let instruction = Instruction::new(0xA,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::LoadIndirect);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: LoadIndirect - d:R[1] set from mem[t:R3]=51736(CA18) where t:R[3]=0014");
                let instruction = Instruction::new(0xB,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::StoreIndirect);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: StoreIndirect - mem[t:R[3]=0014] set from d:R[1]4626(1212)");
                let instruction = Instruction::new(0xC,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::BranchZero);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: BranchZero - pc becomes 14 when d:R[1]=4626(1212) == 0");
                let instruction = Instruction::new(0xD,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::BranchPositive);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: BranchPositive - pc becomes 14 when d:R[1]=4626(1212) > 0");
                let instruction = Instruction::new(0xE,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::JumpRegister);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: JumpRegister - pc becomes 12 from d:R[1]=1212");
                let instruction = Instruction::new(0xF,1,2, 3,20);
                assert_eq!(instruction.op, OpCode::JumpAndLink);
                assert_eq!(instruction.format_for_pp(&machine.regs, &machine.memory),
                           "Op: JumpAndLink - d:R[1] becomes pc and pc becomes 14");
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
                let op_code: OpCode = num::FromPrimitive::from_u8(op).unwrap();
                assert_eq!(op_code, instruction.op);
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
                let opcode: OpCode = num::FromPrimitive::from_u8(0x8).unwrap();
                assert_eq!(opcode, instruction.op);
                assert_eq!(0xA, instruction.d);
                assert_eq!(0xFF, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x12);
                let opcode: OpCode = num::FromPrimitive::from_u8(0x8).unwrap();
                assert_eq!(opcode, instruction.op);
                assert_eq!(0xB, instruction.d);
                assert_eq!(0xFF, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x13);
                let opcode: OpCode = num::FromPrimitive::from_u8(0x7).unwrap();
                assert_eq!(opcode, instruction.op);
                assert_eq!(0xC, instruction.d);
                assert_eq!(0x00, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x14);
                let opcode: OpCode = num::FromPrimitive::from_u8(0x7).unwrap();
                assert_eq!(opcode, instruction.op);
                assert_eq!(0x1, instruction.d);
                assert_eq!(0x01, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x15);
                let opcode: OpCode = num::FromPrimitive::from_u8(0xC).unwrap();
                assert_eq!(opcode, instruction.op);
                assert_eq!(0xA, instruction.d);
                assert_eq!(0x18, instruction.address);
                let instruction = machine.get_next_instruction();
                assert_eq!(machine.pc, 0x16);
                let opcode: OpCode = num::FromPrimitive::from_u8(0x1).unwrap();
                assert_eq!(opcode, instruction.op);
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
                let v: [OpCode; 3] = [
                    num::FromPrimitive::from_u8(0).unwrap(),
                    num::FromPrimitive::from_u8(1).unwrap(),
                    num::FromPrimitive::from_u8(2).unwrap(),
                ];

                let expected: [OpCode; 3] = [
                    OpCode::Halt,
                    OpCode::Add,
                    OpCode::Subtract,
                ];

                for i in 0..3 {
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
                let t_sub: u8 = 4;
                machine.regs[s as usize] = 0x19;
                machine.regs[t as usize] = 0x08;
                machine.regs[t_sub as usize] = 0x20;

                let op: u8 = 1; // add: R[d] <- R[s] + R[t]
                let instruction = Instruction::new(op, d, s, t, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 33u16);

                let op = 2; // subtract: R[d] <- R[s] - R[t]
                let instruction = Instruction::new(op, d, s, t, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 17u16);

                let op = 2; // subtract: R[d] <- R[s] - R[t]
                let instruction = Instruction::new(op, d, s, t_sub, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize] as i16, -7i16);

                let op = 3; // and: R[s] & R[t]
                let instruction = Instruction::new(op, d, s, t, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 8u16);

                let op = 4; // xor: R[d] <- R[s] ^ R[t]
                let instruction = Instruction::new(op, d, s, t, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 17u16);

                let op = 5; // left shift:	R[d] <- R[s] << R[t]
                let instruction = Instruction::new(op, d, s, t, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 6400u16);

                let op = 6; // right shift: R[d] <- R[s] >> R[t]
                let instruction = Instruction::new(op, d, s, t, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 0u16);
            }

            #[test]
            fn load_and_store_instructions() {
                let mut machine = Machine::new();
                let d: u8 = 1;
                let t: u8 = 3;

                let op = 7; // load address	2	R[d] <- addr
                let addr = 0x30;
                let instruction = Instruction::new(op, d, 0, 0, addr);
                assert_eq!(machine.regs[d as usize], 0 as u16);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 0x30 as u16);

                let op = 8; // load	2	R[d] <- mem[addr]
                machine.memory[addr as usize] = 0x1234 as u16;
                let instruction = Instruction::new(op, d, 0, 0, addr);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 0x1234 as u16);

                let op = 9; // store	2	mem[addr] <- R[d]
                let addr = 0x40;
                assert_eq!(machine.memory[addr as usize], 0x0 as u16);
                assert_eq!(machine.regs[d as usize], 0x1234 as u16);
                let instruction = Instruction::new(op, d, 0, 0, addr);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.memory[addr as usize], 0x1234 as u16);

                let op = 0xA; // load indirect	1	R[d] <- mem[R[t]]
                machine.regs[t as usize] = 0x50;
                assert_eq!(machine.memory[machine.regs[t as usize] as usize], 0x0 as u16);
                machine.memory[machine.regs[t as usize] as usize] = 0x1212;
                machine.regs[d as usize] = 0x0;
                assert_eq!(machine.regs[d as usize], 0x0 as u16);
                let instruction = Instruction::new(op, d, 0, t, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 0x1212 as u16);

                let op = 0xB; // store indirect	1	mem[R[t]] <- R[d]
                machine.regs[d as usize] = 0x2121;
                machine.regs[t as usize] = 0x60;
                assert_eq!(machine.memory[machine.regs[t as usize] as usize], 0x0 as u16);
                let instruction = Instruction::new(op, d, 0, t, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.memory[machine.regs[t as usize] as usize], 0x2121 as u16);
            }

            #[test]
            fn control_flow_instructions() {
                let mut machine = Machine::new();
                let d: u8 = 1;

                let op = 0xC; // branch zero	2	if (R[d] == 0) pc <- addr
                let addr = 0x30;
                machine.set_program_counter(0x10);
                assert_eq!(machine.pc, 0x10);
                machine.regs[d as usize] = 0x1;
                let instruction = Instruction::new(op, d, 0, 0, addr);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.pc, 0x10);
                machine.regs[d as usize] = 0x0;
                let instruction = Instruction::new(op, d, 0, 0, addr);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.pc, 0x30);

                let op = 0xD; // branch positive	2	if (R[d] > 0) pc <- addr
                let addr = 0x30;
                machine.set_program_counter(0x10);
                assert_eq!(machine.pc, 0x10);
                machine.regs[d as usize] = 0x0;
                let instruction = Instruction::new(op, d, 0, 0, addr);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.pc, 0x10);
                machine.regs[d as usize] = 0x1;
                let instruction = Instruction::new(op, d, 0, 0, addr);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.pc, 0x30);

                let op = 0xE; // jump register	-	pc <- R[d]
                machine.set_program_counter(0x10);
                assert_eq!(machine.pc, 0x10);
                machine.regs[d as usize] = 0x30;
                let instruction = Instruction::new(op, d, 0, 0, 0);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.pc, 0x30);

                let op = 0xF; // jump and link	2	R[d] <- pc; pc <- addr
                let addr = 0x30;
                machine.set_program_counter(0x10);
                assert_eq!(machine.pc, 0x10);
                machine.regs[d as usize] = 0x30;
                let instruction = Instruction::new(op, d, 0, 0, addr);
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[d as usize], 0x10);
                assert_eq!(machine.pc, 0x30);

                let op = 0x0; // halt	-	exit
                let instruction = Instruction::new(op, 0, 0, 0, 0);
                assert!(!machine.execute_next_instruction(&instruction));
            }
        }
        mod interface_with_external_read_write{
            use super::*;
            use super::ExternalEnv;

            #[test]
            fn std_read_just_words() {
                let mut env = ExternalEnv::new(vec![0x1234, 0x2345, 0x3456]);
                assert_eq!(env.get_next_word().unwrap(), 0x1234);
                assert_eq!(env.get_next_word().unwrap(), 0x2345);
                assert_eq!(env.get_next_word().unwrap(), 0x3456);
                assert!(env.get_next_word().is_none());
            }
            #[test]
            fn std_read_to_memory_by_instruction_direct() {
                // 8	load	2	R[d] <- mem[addr]; Note: addr == 255 is special case
                let mut env = ExternalEnv::new(vec![0x1234, 0x2345, 0x3456]);
                let mut machine = Machine::new();
                machine.set_program_counter(0x10);
                let op = 8;
                let d = 1;
                let addr = 0xFF;
                let instruction = Instruction::new(op, d, 0, 0, addr);
                if instruction.is_read_to_memory(&machine.regs) {
                    let read_try = env.get_next_word();
                    assert!(!read_try.is_none());
                    let word = read_try.unwrap();
                    assert_eq!(word, 0x1234);
                    machine.memory[instruction.address as usize] = word
                }
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.memory[0xFF as usize], 0x1234);
                assert_eq!(machine.regs[d as usize], 0x1234);
            }
            #[test]
            fn std_read_to_memory_by_instruction_indirect() {
                // A	load indirect	1	R[d] <- mem[R[t]]; Note: R[t] == 255 is special case
                let mut env = ExternalEnv::new(vec![0x1234, 0x2345, 0x3456]);
                let mut machine = Machine::new();
                machine.set_program_counter(0x10);
                let op = 0xA;
                let d = 1;
                let t = 2;
                let addr = 0xFF;
                machine.regs[t as usize] = addr;
                let instruction = Instruction::new(op, d, 0, t, 0);
                if instruction.is_read_to_memory(&machine.regs) {
                    let read_try = env.get_next_word();
                    assert!(!read_try.is_none());
                    let word = read_try.unwrap();
                    assert_eq!(word, 0x1234);
                    machine.memory[machine.regs[t as usize] as usize] = word;
                }
                assert!(machine.execute_next_instruction(&instruction));
                assert_eq!(machine.regs[t as usize], 0xFF);
                assert_eq!(machine.memory[0xFF as usize], 0x1234);
                assert_eq!(machine.regs[d as usize], 0x1234);
            }
            #[test]
            fn std_write_from_memory_by_instruction() {
                // 9	store	2	mem[addr] <- R[d]; Note: addr == 255 is special case
                let mut env = ExternalEnv::new(vec![]);
                let mut machine = Machine::new();
                machine.set_program_counter(0x10);
                let op = 0x9;
                let d = 1;
                let addr = 0xFF;
                let word_to_write = 0x9876;
                machine.regs[d as usize] = word_to_write;
                let instruction = Instruction::new(op, d, 0, 0, addr);
                assert!(machine.execute_next_instruction(&instruction));
                if instruction.is_write_from_memory(&machine.regs) {
                    env.put_word(machine.memory[0xFF as usize]);
                    assert_eq!(env.peek_at_last_output(), word_to_write);
                }
                assert_eq!(machine.regs[d as usize], word_to_write);
                assert_eq!(machine.memory[0xFF as usize], word_to_write);
            }
            #[test]
            fn std_write_from_memory_by_instruction_indirect() {
                // B	store indirect	1	mem[R[t]] <- R[d]; Note: R[T] == 255 is special case
                let mut env = ExternalEnv::new(vec![]);
                let mut machine = Machine::new();
                machine.set_program_counter(0x10);
                let op = 0xB;
                let d = 1;
                let t = 2;
                let addr = 0xFF;
                machine.regs[t as usize] = addr;
                let word_to_write = 0x9876;
                machine.regs[d as usize] = word_to_write;
                let instruction = Instruction::new(op, d, 0, t, 0);
                assert!(machine.execute_next_instruction(&instruction));
                if instruction.is_write_from_memory(&machine.regs) {
                    env.put_word(machine.memory[0xFF as usize]);
                    assert_eq!(env.peek_at_last_output(), word_to_write);
                }
                assert_eq!(machine.regs[d as usize], word_to_write);
                assert_eq!(machine.memory[0xFF as usize], word_to_write);
            }
        }
        mod run_one_step {
            use super::*;

            #[test]
            fn run_one_step_test() {
                let mut env = ExternalEnv::new(vec![30, 40]);
                let mut machine = loaded_machine();
                let expected = [0x8AFF, 0x8BFF, 0x7C00, 0x7101, 0xCA18,
                    0x1CCB, 0x2AA1, 0xC014, 0x9CFF, 0x0000];
                for i in 0..expected.len() {
                    assert_eq!(machine.get_memory_word(16 + i), expected[i]);
                }
                machine.set_running();
                machine.set_program_counter(0x10);
                assert_eq!(machine.get_program_counter(), 0x10);
                machine.run_one_step(&mut env, false);
                assert!(machine.get_running());
                assert_eq!(machine.get_program_counter(), 0x11);
                while machine.get_running() {
                    machine.run_one_step(&mut env, false);
                }
                assert_eq!(env.peek_at_last_output(), 1200);
            }
        }
    }
}