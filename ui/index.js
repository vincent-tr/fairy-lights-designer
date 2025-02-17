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
  const workspace = Blockly.inject('blockly', { toolbox });

  const loadButton = document.getElementById('load');
  const saveButton = document.getElementById('save');

  loadButton.addEventListener('click', async () => {
    const file = await open({ accept: '.json' });
    const text = await file.text();
    const state = JSON.parse(text);

    Blockly.serialization.workspaces.load(state, workspace);
  });

  saveButton.addEventListener('click', async () => {
    const state = Blockly.serialization.workspaces.save(workspace);
    download('workspace.json', JSON.stringify(state));
  });

  function download(filename, text) {
    const fileBlob = new Blob([text], { type: 'application/octet-binary' })
    const url = URL.createObjectURL(fileBlob)
  
    const link = document.createElement('a')
    link.setAttribute('href', url)
    link.setAttribute('download', filename)
  
    if (document.createEvent) {
      const event = document.createEvent('MouseEvents')
      event.initEvent('click', true, true)
      link.dispatchEvent(event)
    } else {
      link.click()
    }
  
    // Deallocate resources
    if (URL.revokeObjectURL)
      URL.revokeObjectURL(url)
  }
  
  function open(options = {}) {
    return new Promise((resolve, reject) => {
      const input = document.createElement('input')
  
      if (options.multiple)
        input.setAttribute('multiple', '')
  
      if (options.accept)
        input.setAttribute('accept', options.accept)
  
      input.setAttribute('type', 'file')
      input.style.display = 'none'
  
      input.addEventListener('change', ev => {
        if (input.files.length) {
          if (options.multiple)
            resolve(input.files)
          else
            resolve(input.files[0])
        } else {
          reject(ev)
        }
        input.remove()
      })
  
      document.body.appendChild(input)
  
      const event = document.createEvent('MouseEvent')
      event.initMouseEvent('click', false, false, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null)
      input.dispatchEvent(event)
    })
  }
}
