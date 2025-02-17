import * as wasm from "../runtime/pkg/runtime";

import * as Blockly from "blockly";

import toolbox from './toolbox';
import blocks from './blocks';

setup_wasm();
setup_blockly();

function setup_wasm() {

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
}

function setup_blockly() {
  Blockly.defineBlocksWithJsonArray(blocks);

  Blockly.inject('blockly', {
    toolbox,
  })
}