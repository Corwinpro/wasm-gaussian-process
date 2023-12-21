import * as wasm from "gp-example";
import { memory } from "gp-example/gp_example_bg.wasm";

const sampler = wasm.Sampler.new([0.2, 4.2], [-5.0, 5.0]);

const canvas = document.getElementById("gp-canvas");
const ctx = canvas.getContext("2d");
canvas.height = 600;
canvas.width = 1200;

const drawCurve = (points, tension, colour, lineWidth) => {
    ctx.beginPath();
    ctx.moveTo(points[0].x, points[0].y);

    for (var i = 0; i < points.length - 1; i++) {
        var p0 = i > 0 ? points[i - 1] : points[0];
        var p1 = points[i];
        var p2 = points[i + 1];
        var p3 = i != points.length - 2 ? points[i + 2] : p2;

        var cp1x = p1.x + ((p2.x - p0.x) / 6) * tension;
        var cp1y = p1.y + ((p2.y - p0.y) / 6) * tension;

        var cp2x = p2.x - ((p3.x - p1.x) / 6) * tension;
        var cp2y = p2.y - ((p3.y - p1.y) / 6) * tension;

        ctx.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, p2.x, p2.y);
    }
    ctx.strokeStyle = colour;
    ctx.lineWidth = lineWidth;
    ctx.stroke();
};

const sampleInputs = sampler.get_grid();

const drawGP = () => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for (var i = 0; i < 50; i++) {
        const samples = sampler.get_samples();
        const points = Array.from(samples, (value, index) => ({
            x: sampleInputs[index] * 250,
            y: value * 25 + 300,
        }));
        const randomColor = "#" + Math.floor(Math.random() * 16777215).toString(16);

        drawCurve(points, 1.0, randomColor, 0.3);
    }

    const inputs = sampler.get_inputs();
    const outputs = sampler.get_outputs();
    ctx.beginPath();
    for (var i = 0; i < inputs.length; i++) {
        ctx.fillRect(inputs[i] * 250, outputs[i] * 25 + 300, 10, 10);
    }
    ctx.stroke();

    ctx.beginPath();
    const mean = sampler.mean();
    const variance = sampler.variance();
    const mean_plus_var = Array.from(mean, (value, index) => ({
        x: sampleInputs[index] * 250,
        y: (value + variance[index] * 3.0) * 25 + 300,
    }));
    const mean_minus_var = Array.from(mean, (value, index) => ({
        x: sampleInputs[index] * 250,
        y: (value - variance[index] * 3.0) * 25 + 300,
    }));
    drawCurve(mean_plus_var, 1.0, "black", 0.8);
    drawCurve(mean_minus_var, 1.0, "black", 0.8);
};

drawGP();

canvas.addEventListener("click", (event) => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    console.log(canvasLeft / 250, (canvasTop - 150) / 25);

    ctx.beginPath();
    ctx.fillRect(canvasLeft, canvasTop, 10, 10);
    ctx.stroke();

    sampler.add_samples([canvasLeft / 250], [(canvasTop - 300) / 25]);
    drawGP();
});
