mod utils;

use wasm_bindgen::prelude::*;
use machine::machine::Machine;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
struct interface {
    regs: Vec<u16>,
    memory: Vex<u16>,
    pc: u8,
}

#[wasm_bindgen]
impl interface {
    pub fn new(machine: &Machine) -> Self {
        let regs = machine.get_regs().clone();
        let memory = machine.get_memory().clone();
        let pc = machine.get_program_counter();
        Self {
            regs,
            memory,
            pc,
        }
    }
    #[wasm_bindgen]
    pub fn dump_regs(&self) -> String {
        let mut string= self.regs.map(|r| format!("R[{}"))
        String::from("done")
    }
}

#[wasm_bindgen]

