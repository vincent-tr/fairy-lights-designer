import * as Blockly from 'blockly';

export const generator = new Blockly.Generator('fairy-lights-runtime');

generator.forBlock['sample_block'] = function(block, generator) {
  return 'my code string';
};