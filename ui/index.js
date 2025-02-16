import * as wasm from "../runtime/pkg/runtime";
import * as Blockly from "blockly/core";
import "blockly/blocks";
import "blockly/msg/en"
import {registerFieldColour, FieldColour} from '@blockly/field-colour';

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
  registerFieldColour();

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
    }
  ]);
    


  const toolbox = {
    // There are two kinds of toolboxes. The simpler one is a flyout toolbox.
    kind: 'flyoutToolbox',
    // The contents is the blocks and other items that exist in your toolbox.
    contents: [
      {
        kind: 'block',
        type: 'set',
      },
      {
        kind: 'block',
        type: 'controls_if'
      },
      {
        kind: 'block',
        type: 'controls_whileUntil'
      }
      // You can add more blocks to this array.
    ]
  };

  Blockly.inject('blockly', {
    toolbox: toolbox,
  })
}