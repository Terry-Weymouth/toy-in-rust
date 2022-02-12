import * as wasm from "toy-wasm";

function regs_header_to_table(table) {
    let row_labels = ["Regs","0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "5"]
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

function regs_data_to_table(table_row, string_array) {
    for(var i = 0; i < 16; i++) {
        var td = document.createElement("TD");
        var text = document.createTextNode(string_array[i]);
        td.appendChild(text);
        table_row.appendChild(td);
    }
}

function button_clicked() {
    console.log("button clicked");
}

let button = document.getElementById("button");
console.log(button)
button.addEventListener("click", button_clicked);

let portal = wasm.Portal.new();
portal.load_regs([9, 8, 7, 6, 5, 4, 3, 2, 1]);
let regs_data = collect_regs_data(portal);

console.log(document.getElementById("regsTable"))
const table = document.getElementById("regsTable");
regs_header_to_table(table);
regs_data_to_table(table.rows[1], regs_data);

