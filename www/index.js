import * as wasm from "toy-wasm";

let previous_pc = 0;
let pc = 0;
let portal = wasm.Portal.new();
let running_delay = 1000;

function regs_header_to_table(table) {
    let row_labels = ["Regs","0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"]
    var row = document.createElement("TR");
    for(var i = 0; i < 17; i++) {
        var th = document.createElement("TH");
        var text = document.createTextNode(row_labels[i]);
        th.appendChild(text);
        row.appendChild(th);
    }
    table.appendChild(row);

    var row = document.createElement("TR");
    var th = document.createElement("TH");
    var text = document.createTextNode("Values");
    th.appendChild(text);
    row.appendChild(th);
    table.appendChild(row);
}

function collect_regs_data(portal) {
    let regs_array = []
    for(var i = 0; i < 16; i++){
        let str = portal.reg_as_string(i);
        regs_array.push(str);
    }
    return regs_array;
}

function regs_add_data_to_table(table_row, string_array) {
    for(var i = 0; i < 16; i++) {
        var td = document.createElement("TD");
        var text = document.createTextNode(string_array[i]);
        td.appendChild(text);
        table_row.appendChild(td);
    }
}

function regs_update_data_in_table(table_row, string_array) {
    for(var i = 0; i < 16; i++) {
        table_row.cells[i + 1].innerHTML = string_array[i];
    }
}

function memory_header_to_table(table) {
    let row_labels = ["Memory","0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"]
    var row = document.createElement("TR");
    for(var i = 0; i < 17; i++) {
        var th = document.createElement("TH");
        var text = document.createTextNode(row_labels[i]);
        th.appendChild(text);
        row.appendChild(th);
    }
    table.appendChild(row);

    for (var index = 0; index < 256; index += 16) {
        var header = ("0000" + index.toString(16)).substr(-4);
        header = index.toString(16).toUpperCase().padStart(4, "0");
        var row = document.createElement("TR");
        var th = document.createElement("TH");
        var text = document.createTextNode(header + ":");
        th.appendChild(text);
        row.appendChild(th);
        table.appendChild(row);
    }
}

function collect_memory_data(portal) {
    let memory_array = []
    for(var i = 0; i < 256; i++){
        let str = portal.memory_as_string(i);
        memory_array.push(str);
    }
    return memory_array;
}

function memory_add_data_to_table(table_row, start_index, string_array) {
    for(var i = 0; i < 16; i++) {
        var td = document.createElement("TD");
        var text = document.createTextNode(string_array[start_index + i]);
        td.appendChild(text);
        table_row.appendChild(td);
    }
}

function memory_update_data_in_table(table_row, start_index, string_array) {
    for(var i = 0; i < 16; i++) {
        table_row.cells[i + 1].innerHTML = string_array[start_index + i];
    }
}

function pc_indicator(table, address, set) {
    const row_index = Math.floor(address/16);
    const col_index = address % 16;
    let color = "white";
    if (set) {
        color = "red";
    }
    table.rows[row_index + 1].cells[col_index + 1].style.backgroundColor = color;
}

function set_up_display() {
    let regs_data = collect_regs_data(portal);
    let memory_data = collect_memory_data(portal);

    let table = document.getElementById("regsTable");
    regs_header_to_table(table);
    regs_add_data_to_table(table.rows[1], regs_data);

    table = document.getElementById("memoryTable");
    memory_header_to_table(table);
    for (var i = 0; i < 16; i++) {
        memory_add_data_to_table(table.rows[1 + i], i*16, memory_data);
    }
    pc = portal.get_pc();
    pc_indicator(table, pc, true);
    previous_pc = pc;
}

function refresh_display() {
    let regs_data = collect_regs_data(portal);
    let memory_data = collect_memory_data(portal);

    let table = document.getElementById("regsTable");
    regs_update_data_in_table(table.rows[1], regs_data);

    table = document.getElementById("memoryTable");
    for (var i = 0; i < 16; i++) {
        memory_update_data_in_table(table.rows[1 + i], i*16, memory_data);
    }

    let running = portal.get_program_running();
    step_button.disabled = !running;
    run_button.disabled = !running;
    let running_text = "running";
    if (!running) {
        running_text = "not " + running_text;
    }
    document.getElementById("status").innerHTML = running_text;
    document.getElementById("input_values").innerHTML =
        portal.inputs_as_string();
    document.getElementById("output_values").innerHTML =
        portal.outputs_as_string();
    document.getElementById("instruction").innerHTML =
        portal.next_instruction_as_string();

    pc = portal.get_pc();
    pc_indicator(table, previous_pc, false);
    pc_indicator(table, pc, true);
    previous_pc = pc;
}

function step_on_click() {
    portal.step_program();
    refresh_display();
}
function run_on_click() {
    run_button.disabled = true
    let running = portal.get_program_running();
    if (running){
        portal.step_program();
        refresh_display();
    }
    running = portal.get_program_running();
    if (running) {
        console.log("running delay", running_delay)
        setTimeout(run_on_click, running_delay);
    }
}

function restart() {
    portal.clear_regs_and_memory();
    portal.load_fixed_program();
    portal.set_pc(0x10);
    portal.push_to_input(2);
    portal.push_to_input(3);
    portal.set_program_running();
    refresh_display();
}

let step_button = document.getElementById("step");
step_button.addEventListener("click", step_on_click);
let run_button = document.getElementById("run");
run_button.addEventListener("click", run_on_click);
let restart_button = document.getElementById("restart");
restart_button.addEventListener("click", restart);

portal.load_fixed_program();
portal.set_pc(0x10);
portal.push_to_input(2);
portal.push_to_input(3);
portal.set_program_running();
set_up_display();
refresh_display();
