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
    ctx.fillStyle = "#9040f0";
    ctx.beginPath();
    ctx.arc(x, y, r, 0, 2 * Math.PI);
    ctx.fill();
}

function renderShip(x, y, r, o) {
    ctx.fillStyle = "#a050f0";
    ctx.beginPath();
    ctx.arc(x, y, r, o - 0.85*Math.PI, o + 0.85*Math.PI, true);
    ctx.lineTo(x, y);
    ctx.fill();
}

let arena = wasm.uncommon_priors_require_origin_disputes();
arena.tick();

const renderLoop = () => {
    arena.tick();
    space();
    let entityCount = arena.entity_count();
    for (let i=0; i<entityCount; i++) {
        let x = arena.entity_render_instruction_x(i);
        let y = arena.entity_render_instruction_y(i);
        let r = arena.entity_render_instruction_r(i);
        let o = arena.entity_render_instruction_o(i);
        renderShip(x, y, r, o);
    }
    requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);

addEventListener('keydown', keyHandler);

function keyHandler(event) {
    if (event.code == "ArrowLeft") {
        console.log("left!");
        arena.input_left();
    }
    if (event.code == "ArrowRight") {
        console.log("right!");
        arena.input_right();
    }
    if (event.code == "ArrowUp") {
        console.log("thrust!");
        arena.input_thrust();
    }
}
