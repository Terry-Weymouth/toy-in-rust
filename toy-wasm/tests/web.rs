//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use toy_wasm::Portal;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn reg_dump() {
    let mut portal = Portal::new();
    portal.load_regs(vec![0x111, 0x222, 0x1234, 0, 0x555, 0x666, 0x777]);
    assert_eq!(portal.reg_as_string(1), "0222");
    assert_eq!(portal.reg_as_string(2), "1234");
    assert_eq!(portal.reg_as_string(3), "0000");
    assert_eq!(portal.reg_as_string(4), "0555");
}

#[wasm_bindgen_test]
fn program_counter() {
    let mut portal = Portal::new();
    portal.set_pc(25);
    assert_eq!(portal.get_pc(), 25);
}

#[wasm_bindgen_test]
fn load_program() {
    let mut portal = Portal::new();
    portal.load_fixed_program();
    let value = portal.memory_as_string(0x10);
    assert_eq!(value, "8AFF");
}

#[wasm_bindgen_test]
fn step_program() {
    let mut portal = Portal::new();
    portal.load_fixed_program();
    portal.set_pc(0x10);
    portal.push_to_input(20);
    portal.push_to_input(30);
    let value = portal.memory_as_string(0xFF);
    assert_eq!(value, "0000");
    portal.set_program_running();
    portal.step_program();
    let running = portal.get_program_running();
    assert_eq!(running, true);
    let value = portal.memory_as_string(0xFF);
    assert_eq!(value, "0014");
}