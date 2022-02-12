mod utils;

use wasm_bindgen::prelude::*;
use machine::machine::Machine as Toy;
use machine::program_reader::program_reader::ProgramReader;
use serde::Serialize;

#[wasm_bindgen]
pub struct Portal {
    backing: Toy,
    machine: Machine,
}

#[derive(Clone, Debug, Serialize)]
pub struct Machine {
    pub regs: Vec<i32>,
    pub memory: Vec<i32>,
    pub pc: i32,
}

#[wasm_bindgen]
impl Portal {
    pub fn new() -> Self {
        let backing = Toy::new();
        let regs = backing.get_regs().iter().map(|&value| value as i32).collect();
        let memory = backing.get_memory().iter().map(|&value| value as i32).collect();
        let pc = backing.get_program_counter() as i32;
        let machine = Machine {
            regs,
            memory,
            pc,
        };
        Self {
            backing,
            machine,
        }
    }

    pub fn load_regs(&mut self, regs: Vec<i32>) {
        for i in 0..16.min(regs.len()){
            self.machine.regs[i] = regs[i];
            self.backing.set_reg(i, regs[i] as u16);
        }
    }

    pub fn reg_as_string(&self, index: usize) -> String {
        let value = self.machine.regs[index];
        format!("{:04X}", value)
    }

    pub fn memory_as_string(&self, index: usize) -> String {
        let value = self.machine.memory[index];
        format!("{:04X}", value)
    }

    pub fn get_pc(&self) -> i32 {
        self.machine.pc
    }

    pub fn set_pc(&mut self, value: i32){
        self.machine.pc = value;
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
        self.machine.regs = self.backing.get_regs().iter().map(|&value| value as i32).collect();
        self.machine.memory = self.backing.get_memory().iter().map(|&value| value as i32).collect();
        self.machine.pc = self.backing.get_program_counter() as i32;
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
