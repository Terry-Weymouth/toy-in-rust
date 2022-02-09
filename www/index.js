import * as wasm from "toy-wasm";
var canvas = document.getElementById('display');
canvas.style.border = '1px solid #000'
var ctx = canvas.getContext('2d');

function draw_text_with_box(text, x, y) {
    ctx.fillText(text, x, y);
    const metrics = ctx.measureText(text);
    const width = metrics.width;
    const actualHeight = metrics.actualBoundingBoxAscent + metrics.actualBoundingBoxDescent;
    const fontHeight = metrics.fontBoundingBoxAscent + metrics.fontBoundingBoxDescent;
    const height = actualHeight + 4;
    let bx = x;
    let by = y + 6;
    ctx.moveTo(bx, by);
    ctx.lineTo(bx+width, by);
    ctx.lineTo(bx+width, by-height);
    ctx.lineTo(bx, by-height);
    ctx.lineTo(bx, by);
    ctx.stroke();
    let next_x = x + width + 10;
    return next_x;
}

function draw_regs(string_array) {
    ctx.fillStyle = 'rgb(200, 0, 0, 0.1)';
    ctx.fillRect(10, 10, 50, 50);

    ctx.fillStyle = 'rgba(0, 255, 200, 0.5)';
    ctx.fillRect(30, 30, 50, 50);

    ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
    ctx.font = "20px Arial";
    var x = 10;
    var y = 50;
    for(var i = 0; i < 16; i++) {
        x = draw_text_with_box(string_array[i], x, y);
        if (((i + 1) % 4) == 0) {
            x = 10;
            y += 30;
        }
    }
}

let portal = wasm.Portal.new();
portal.load_regs([9, 8, 7, 6, 5, 4, 3, 2, 1]);
let regs_strs = []
for(var i = 0; i < 16; i++){
    let str = portal.reg_as_string(i);
    regs_strs.push(str);
}
draw_regs(regs_strs);

// let regs_str = portal.dump_regs();
// wasm.hello(regs_str);

