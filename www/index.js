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

function renderCircle(x, y, r, color) {
    ctx.fillStyle = color;
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

function renderHealthbar(x, y, hp) {
    ctx.fillStyle = "#ff0000";
    ctx.fillRect(x - 10, y - 20, 20, 3);
    ctx.fillStyle = "#00ff00";
    ctx.fillRect(x - 10, y - 20, (hp/100)*20, 3);
}

let arena = wasm.uncommon_priors_require_origin_disputes(0);
let scenarios = {
    'space': 0,
    'patrol-fleet': 1,
    'turret': 2,
    'single-adversary': 3
};
for (let scenarioName in scenarios) {
    document.getElementById(scenarioName).addEventListener(
        'click',
        function () {
            arena = wasm.uncommon_priors_require_origin_disputes(
                scenarios[scenarioName]
            );
        }
    );
}


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
            renderHealthbar(x, y, s);
            let dash = document.getElementById("heroine-dash");
            dash.textContent = `x=${x.toFixed(2)} y=${y.toFixed(2)} θ=${o.toFixed(2)} shields=${s.toFixed(1)}`;
            break;
        case 2: // other ship
            renderShip(x, y, r, o, "#c0c0c0");
            renderHealthbar(x, y, s);
            break;
        case 3: // inactive/unready torpedo
            renderCircle(x, y, r, "#505050");
            break;
        case 4: // active/ready torpedo
            renderCircle(x, y, r, "#ffffff");
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
