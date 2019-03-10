import * as wasm from "uss_uncommon_priors";

let canvas = document.createElement("canvas");
canvas.setAttribute("width", 600);
canvas.setAttribute("height", 400);
let body = document.getElementsByTagName("body")[0];
body.appendChild(canvas);
let ctx = canvas.getContext("2d");

function renderCircle(x, y, r) {
    ctx.fillStyle = "#9040f0";
    ctx.beginPath();
    ctx.arc(x, y, r, 0, 2 * Math.PI, false);
    ctx.fill();
}

wasm.rah();
renderCircle(200, 200, 30);
