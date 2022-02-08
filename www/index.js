import * as wasm from "toy-wasm";

let regs = wasm.dump_regs();
wasm.hello(regs);
