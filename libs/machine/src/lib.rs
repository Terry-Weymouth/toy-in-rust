#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

#[derive(Debug)]
pub struct InputHolder {
    input_buffer: Vec<u16>,
}

#[derive(Debug)]
pub struct OutputHolder {
    output_buffer: Vec<u16>,
}

#[derive(Debug)]
pub struct Machine {
    pc: u8,
    regs: [u16; 16],
    memory: [u16; 256],
    input: InputHolder,
    output: OutputHolder,
}

#[derive(Debug)]
pub struct Instruction {
    op:    u8, // really u4 - one hex digit
    d:     u8, // really u4 - one hex digit
    s:     u8, // really u4 - one hex digit
    t:     u8, // really u4 - one hex digit
    address: u8,
}

impl InputHolder{
    fn new() -> Self{
        let input_buffer = Vec::new();
        Self {
            input_buffer,
        }
    }
    fn get_current_word(&mut self) -> Option<u16> {
        if self.input_buffer.len() == 0 {
            return None
        }
        let value = self.input_buffer.remove(0);
        Option::from(value)
    }
}

impl OutputHolder{
    fn new() -> Self{
        let output_buffer = Vec::new();
        Self {
            output_buffer,
        }
    }
}

impl Machine {
    fn new(program: &str) -> Self {
        print!("{}",program);
        let pc: u8 = 0;
        let memory: [u16; 256] = [0; 256];
        let regs: [u16; 16] = [0; 16];
        let input= InputHolder::new();
        let output = OutputHolder::new();
        Self {
            pc,
            regs,
            memory,
            input,
            output,
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
    fn read_word_into_memory_255(&mut self) -> u16 {
        let value = &self.input.get_current_word();
        let mut ret: u16 = 0;
        match value {
            Some(value) => {
                let mem = *value;
                println!{"memory inside {:x}", self.memory[255]};
                self.memory[255] = mem;
                println!{"memory inside {:x}", self.memory[255]};
                ret = mem;
            },
            None =>{
            }
        };
        ret
    }
    fn write_word_from_memory_255(&self) {
        todo!()
    }
    fn alu_operation(&self, _instruction: &Instruction) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod read_write_memory_255 {
        use super::*;

        impl Machine {
            fn setup_for_test() -> Self {
                let pc: u8 = 0;
                let memory: [u16; 256] = [0; 256];
                let regs: [u16; 16] = [0; 16];
                let mut input = InputHolder::new();
                let output = OutputHolder::new();
                let input_data = vec![0x1234 as u16, 0x2345, 0x3456, 0x4567];
                for i in input_data{
                    input.input_buffer.push(i);
                }
                Self {
                    pc,
                    regs,
                    memory,
                    input,
                    output,
                }
            }
        }

        #[test]
        fn read() {
            let mut machine = Machine::setup_for_test();
            println!("memory before {:x}", machine.memory[225]);
            let ret = &machine.read_word_into_memory_255();
            println!("memory after {:x}", machine.memory[225]);
            let expected =  0x1234 as u16;
            assert_eq!(*ret, expected);
            assert_eq!(machine.memory[225], expected)
        }
    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn change_vec_at_index() {
        struct Thing {
            holder: Vec<u8>,
        }
        impl Thing{
            fn new() -> Self{
                let holder = vec![1, 2, 3, 4, 5, 6, 7, 8];
                Self {
                    holder,
                }
            }
            fn set_value_at(&mut self, value: u8, index: usize){
                std::mem::replace(&mut self.holder[index],value);
            }
        }
        let mut thing = Thing::new();
        assert_eq!(thing.holder[5], 6);
        let value = 24 as u8;
        thing.set_value_at(value, 5);
        assert_eq!(thing.holder[5], 24);
    }

    #[test]
    fn change_array_at_index() {
        struct Thing {
            holder: [i32; 8],
        }
        impl Thing{
            fn new() -> Self{
                let holder: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
                Self {
                    holder,
                }
            }
            fn set_value_at(&mut self, value: i32, index: usize){
                std::mem::replace(&mut self.holder[index],value);
            }
        }
        let mut thing = Thing::new();
        assert_eq!(thing.holder[5], 6);
        let value = 24;
        thing.set_value_at(value, 5);
        assert_eq!(thing.holder[5], 24);
    }
}
