//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use toy_wasm::Interface;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn reg_dump() {
    let mut machine_interface = Interface::new();
    machine_interface.load_regs(vec![0, 0, 0, 0, 5, 6, 777, 0, 0, 10, 0, 0, 0, 0, 0, 0]);
    let reg_string = machine_interface.dump_regs();
    assert_eq!(reg_string, "R[0]=0R[1]=0R[2]=0R[3]=0R[4]=0R[5]=0R[6]=0R[7]=0R[8]=0R[9]=0R[10]=0R[11]=0R[12]=0R[13]=0R[14]=0R[15]=0");
}
