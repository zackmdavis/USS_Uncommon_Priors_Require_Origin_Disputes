import * as wasm from "uss_uncommon_priors";

let canvas = document.createElement("canvas");
canvas.setAttribute("width", 600);
canvas.setAttribute("height", 400);

let body = document.getElementsByTagName("body")[0];
body.appendChild(canvas);

let ctx = canvas.getContext("2d");

function space() {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
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
        switch (kind) {
        case 1: // our heroine
            renderShip(x, y, r, o, "#a050f0");
            break;
        case 2: // other ship
            renderShip(x, y, r, o, "#c0c0c0");
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
    console.log(event.code);
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
