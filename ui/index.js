import * as wasm from "../runtime/pkg/runtime";

import * as Blockly from "blockly";

import toolbox from './toolbox';

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
  Blockly.defineBlocksWithJsonArray([
    {
      "type": "set",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Set light color\nIndex = %1\nRed = %2 Green = %3 Blue = %4",
      "args0": [
        {
          "type": "input_value",
          "name": "index",
          "check": "Number"
        },
        {
          "type": "input_value",
          "name": "r",
          "check": "Number"
        },
        {
          "type": "input_value",
          "name": "g",
          "check": "Number"
        },
        {
          "type": "input_value",
          "name": "b",
          "check": "Number"
        }
      ],
      "previousStatement": null,
      "nextStatement": null,
      "colour": 90
    },
    {
      "type": "sleep",
      "tooltip": "",
      "helpUrl": "",
      "message0": "sleep %1ms",
      "args0": [
        {
          "type": "input_value",
          "name": "delay",
          "check": "Number"
        }
      ],
      "colour": 90
    }
  ]);

  Blockly.inject('blockly', {
    toolbox,
  })
}