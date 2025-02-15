import * as wasm from "../runtime/pkg/runtime";

const WIDTH = 1000;
const HEIGHT = 1000;

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const data = wasm.init();
const imageData = new ImageData(data, WIDTH, HEIGHT);

requestAnimationFrame(render);

function render() {
  wasm.render();

  ctx.putImageData(imageData, 0, 0);

  requestAnimationFrame(render);
}
