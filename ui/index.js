import * as wasm from '../runtime/pkg/runtime';

import * as Blockly from 'blockly';

import toolbox from './toolbox';
import blocks from './blocks';
import { generator } from './generator';
import * as api from './api';

setupWasm();
setupBlockly();
setupManagement();

window.fairy_api = api;

function setupWasm() {

  const WIDTH = 1000;
  const HEIGHT = 1000;
  
  const canvas = document.getElementById("render");
  const ctx = canvas.getContext("2d");
  
  wasm.init();
  
  requestAnimationFrame(render);
  
  function render() {
    const data = wasm.render();
    const imageData = new ImageData(data, WIDTH, HEIGHT);
    ctx.putImageData(imageData, 0, 0);

    // console.log('running', wasm.running());
  
    requestAnimationFrame(render);
  }
}

let blocklyLoading = false;

function setupBlockly() {
  // Disable 'set variable to'
  Blockly.Blocks['math_change'] = null;

  Blockly.defineBlocksWithJsonArray(blocks);
  const workspace = Blockly.inject('blockly', { toolbox });

  workspace.addChangeListener((event) => {
    if (event.type === Blockly.Events.FINISHED_LOADING) {
      blocklyLoading = false;
      return;
    }

    if (!event.isUiEvent && !blocklyLoading) {
      onUpdate();
    }
  });

  const runButton = document.getElementById('run');

  runButton.addEventListener('click', () => {
    run(workspace);
  });
}

function run(workspace) {
  Blockly.Variables.allUsedVarModels(workspace).forEach(variable => {
    console.log("variable: ", variable.getId(), variable.name);
  });

  generator.init(workspace);
  try {
    const blocks = workspace.getTopBlocks(true);
    if (blocks.length !== 1) {
      throw new Error('Only one top block allowed');
    }
  } finally {
    generator.finish();
  }

  const variables = Blockly.Variables.allUsedVarModels(workspace).map(variable => variable.name);
  const body = JSON.parse(generator.workspaceToCode(workspace));
  const ast = { variables, body };

  console.log('AST', ast);

  const bytecode = wasm.compile(JSON.stringify(ast));

  const textbox = document.getElementById('bytecode');
  textbox.value = bytecode;

  console.log('Bytecode', bytecode);

  wasm.execute(bytecode);
}

function setupManagement() {
  const list = document.getElementById('list');
  list.addEventListener('change', onSelect);

  const newButton = document.getElementById('new');
  newButton.addEventListener('click', onNew);

  const deleteButton = document.getElementById('delete');
  deleteButton.addEventListener('click', onDelete);

  const duplicateButton = document.getElementById('duplicate');
  duplicateButton.addEventListener('click', onDuplicate);

  const name = document.getElementById('name');
  name.addEventListener('change', onUpdate);

  runAsync(async () => {
    await refreshList();
    await setCurrent(list.value);
  });
}

function onSelect() {
  const list = document.getElementById('list');
  const id = list.value;

  setCurrent(id);
}

function onNew() {
  const name = 'New program';
  const content = '{}';

  runAsync(async () => {
    const id = await api.create(name, content);
    await refreshList();
    await setCurrent(id);
  });
}

function onUpdate() {
  const id = document.getElementById('list').value;
  const name = document.getElementById('name').value;
  const content = Blockly.serialization.workspaces.save(Blockly.getMainWorkspace());

  runAsync(async () => {
    await api.update(id, name, content);
    await refreshList();
    await setCurrent(id);
  });
}

function onDelete() {
  const id = document.getElementById('list').value;
  const name = document.getElementById('name').value;

  if (!confirm(`Are you sure you want to delete this program?\n\n  ${name}`)) {
    return;
  }

  runAsync(async () => {
    await api.remove(id);
    await refreshList();
    await setCurrent(list.value);
  });
}

function onDuplicate() {
  const name = document.getElementById('name').value + ' (copy)';
  const content = Blockly.serialization.workspaces.save(Blockly.getMainWorkspace());

  runAsync(async () => {
    const id = await api.create(name, content);
    await refreshList();
    await setCurrent(id);
  });
}

async function setCurrent(id) {
  const item = await api.read(id);

  const list = document.getElementById('list');
  list.value = id;

  const name = document.getElementById('name');
  name.value = item.name;

  const workspace = Blockly.getMainWorkspace();

  blocklyLoading = true;
  try {
    Blockly.serialization.workspaces.load(item.content, workspace);
  } catch (error) {
    blocklyLoading = false;
    throw error;
  }
}

async function refreshList() {
  const list = document.getElementById('list');
  const items = await api.list();

  while (list.options.length) {
    list.remove(0);
  }

  items.forEach(item => {
    const uiItem = document.createElement('option');
    uiItem.textContent = item.name;
    uiItem.value = item.id;
    list.appendChild(uiItem);
  });
}

function runAsync(target) {
  target().catch(error => {
    console.error(error);
  });
}