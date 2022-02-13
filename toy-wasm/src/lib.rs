mod utils;

use wasm_bindgen::prelude::*;
use machine::machine::Machine as Toy;
use machine::program_reader::program_reader::ProgramReader;
use machine::external_env::external_env::ExternalEnv;

#[wasm_bindgen]
pub struct Portal {
    backing: Toy,
    external: ExternalEnv,
}

#[wasm_bindgen]
impl Portal {
    pub fn new() -> Self {
        let backing = Toy::new();
        let external = ExternalEnv::new(vec![]);
        Self {
            backing,
            external,
        }
    }

    pub fn load_regs(&mut self, regs: Vec<i32>) {
        for i in 0..16.min(regs.len()){
            self.backing.set_reg(i, regs[i] as u16);
        }
    }

    pub fn reg_as_string(&self, index: usize) -> String {
        let value = self.backing.get_regs()[index];
        format!("{:04X}", value)
    }

    pub fn memory_as_string(&self, index: usize) -> String {
        let value = self.backing.get_memory_word(index);
        format!("{:04X}", value)
    }

    pub fn inputs_as_string(&self) -> String {
        self.external.input_for_display()
    }

    pub fn outputs_as_string(&self) -> String {
        self.external.output_for_display()
    }

    pub fn next_instruction_as_string(&self) -> String{
        let pc = self.backing.get_program_counter();
        let instruction_word = self.backing.get_memory_word(pc as usize);
        let operation = self.backing.current_instruction_pp(instruction_word);
        format!("{:02X}: {:04X} - {}", pc, instruction_word, operation)
    }

    pub fn get_pc(&self) -> i32 {
        self.backing.get_program_counter().into()
    }

    pub fn set_pc(&mut self, value: i32){
        self.backing.set_program_counter(value as u8);
    }

    pub fn load_fixed_program(&mut self) {
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
        self.backing.load(loads);
    }

    pub fn push_to_input(&mut self, value: i32) {
        self.external.push_to_input(value as u16);
    }

    pub fn set_program_running(&mut self) {
        self.backing.set_running();
    }

    pub fn reset_program_running(&mut self) {
        self.backing.reset_running();
    }

    pub fn get_program_running(&mut self) -> bool {
        self.backing.get_running()
    }

    pub fn step_program(&mut self) {
        self.backing.run_one_step(&mut self.external, false);
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
