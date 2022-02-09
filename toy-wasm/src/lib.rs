mod utils;

use wasm_bindgen::prelude::*;
use machine::machine::Machine as Toy;
use serde::Serialize;

#[wasm_bindgen]
pub struct Interface {
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
impl Interface {
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
            self.machine.memory[i] = regs[i];
            self.backing.set_reg(i, regs[i] as u16);
        }
    }

    pub fn regs(&self) -> Vec<i32> {
        self.machine.regs.clone()
    }

    pub fn dump_regs(&self) -> String {
        let string: String = self.regs().iter().enumerate().
            map(|(i, r)| format!("R[{}]={}", i, r))
            .collect();
        string
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}
