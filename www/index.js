import * as wasm from "uss_uncommon_priors";

let canvas = document.createElement("canvas");
let width = 600;
let height = 400;
canvas.setAttribute("width", width);
canvas.setAttribute("height", height);

let body = document.getElementsByTagName("body")[0];
body.appendChild(canvas);

let ctx = canvas.getContext("2d");

function line(direction, mark) {
    let start_x, start_y, end_x, end_y;
    switch (direction) {
    case "horizontal":
        start_x = 0;
        end_x = width;
        start_y = end_y = mark;
        break;
    case "vertical":
        start_y = 0;
        end_y = height;
        start_x = end_x = mark;
        break;
    }
    ctx.strokeStyle = "#00b0a0";
    ctx.beginPath();
    ctx.moveTo(start_x, start_y);
    ctx.lineTo(end_x, end_y);
    ctx.stroke();
}

function space() {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i<width/100; i++) {
        line("vertical", i*100);
    }
    for (let j = 0; j<height/100; j++) {
        line("horizontal", j*100);
    }
}

function renderCircle(x, y, r) {
    ctx.fillStyle = "#ffffff";
    ctx.beginPath();
    ctx.arc(x, y, r, 0, 2 * Math.PI);
    ctx.fill();
}

function renderShip(x, y, r, o, color) {
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.arc(x, y, r, o - 0.85*Math.PI, o + 0.85*Math.PI, true);
    ctx.lineTo(x, y);
    ctx.fill();
}

let arena = wasm.uncommon_priors_require_origin_disputes();

const renderLoop = () => {
    arena.tick();
    space();
    let entityCount = arena.entity_count();
    for (let i=0; i<entityCount; i++) {
        let kind = arena.entity_render_instruction_kind(i);
        let x = arena.entity_render_instruction_x(i);
        let y = arena.entity_render_instruction_y(i);
        let r = arena.entity_render_instruction_r(i);
        let o = arena.entity_render_instruction_o(i);
        let s = arena.entity_render_instruction_shields(i);
        switch (kind) {
        case 1: // our heroine
            renderShip(x, y, r, o, "#a050f0");
            let dash = document.getElementById("heroine-dash");
            dash.textContent = `x=${x.toFixed(2)} y=${y.toFixed(2)} θ=${o.toFixed(2)} shields=${s.toFixed(2)}`;
            break;
        case 2: // other ship
            renderShip(x, y, r, o, "#c0c0c0");
            let enemy_dash = document.getElementById("enemy-dash");
            enemy_dash.textContent = `x=${x.toFixed(2)} y=${y.toFixed(2)} θ=${o.toFixed(2)} shields=${s.toFixed(2)}`;
            break;
        case 3: // torpedo
            renderCircle(x, y, r);
            break;
        }
    }
    requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);

addEventListener('keydown', keyHandler);

function keyHandler(event) {
    switch (event.code) {
    case "ArrowLeft":
        arena.input_left();
        break;
    case "ArrowRight":
        arena.input_right();
        break;
    case "ArrowUp":
        arena.input_thrust();
        break;
    case "Space":
        arena.input_fire();
        break;
    }
}
