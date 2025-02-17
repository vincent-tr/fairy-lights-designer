import * as Blockly from 'blockly';

const Order = {
  ATOMIC: 0,
};

export const generator = new Blockly.Generator('fairy-lights-runtime');

generator.forBlock['controls_if'] = function(block, generator) {
  throw new Error('Not implemented: controls_if');
};

generator.forBlock['logic_compare'] = function(block, generator) {
  throw new Error('Not implemented: logic_compare');
};

generator.forBlock['logic_operation'] = function(block, generator) {
  throw new Error('Not implemented: logic_operation');
}

generator.forBlock['logic_negate'] = function(block, generator) {
  throw new Error('Not implemented: logic_negate');
}

generator.forBlock['logic_boolean'] = function(block, generator) {
  const value = block.getFieldValue('BOOL') === 'TRUE';
  return [
    `{ "type": "literal", "value": ${value} }`,
    Order.ATOMIC
  ];
}

generator.forBlock['logic_ternary'] = function(block, generator) {
  throw new Error('Not implemented: logic_ternary');
}

generator.forBlock['controls_repeat_ext'] = function(block, generator) {
  throw new Error('Not implemented: controls_repeat_ext');
}

generator.forBlock['controls_whileUntil'] = function(block, generator) {
  const type = block.getFieldValue('MODE') === 'UNTIL' ? 'until' : 'while';
  const cond = generator.valueToCode(block, 'BOOL', Order.ATOMIC) || 'false';
  const body = generator.statementToCode(block, 'DO');

  return `{ "type": "${type}", "condition": ${cond}, "body": ${body} }`;
}

generator.forBlock['controls_for'] = function(block, generator) {
  throw new Error('Not implemented: controls_for');
}

generator.forBlock['controls_flow_statements'] = function(block, generator) {
  throw new Error('Not implemented: controls_flow_statements');
}

generator.forBlock['math_number'] = function(block, generator) {
  const value = block.getFieldValue('NUM');
  return [
    `{ "type": "literal", "value": ${value} }`,
    Order.ATOMIC
  ];
}

generator.forBlock['math_arithmetic'] = function(block, generator) {
  throw new Error('Not implemented: math_arithmetic');
}

generator.forBlock['math_modulo'] = function(block, generator) {
  throw new Error('Not implemented: math_modulo');
}

generator.forBlock['math_constrain'] = function(block, generator) {
  throw new Error('Not implemented: math_constrain');
}

generator.forBlock['math_random_int'] = function(block, generator) {
  throw new Error('Not implemented: math_random_int');
}

generator.forBlock['variables_get'] = function(block, generator) {
  return `{ "type": "get_variable", "id": "${block.getFieldValue('VAR')}" }`;
}

generator.forBlock['variables_set'] = function(block, generator) {
  const name = block.getFieldValue('VAR');
  const value = generator.valueToCode(block, 'VALUE', Order.ATOMIC);
  return `{ "type": "set_variable", "id": "${name}", "value": ${value} }`;
}

generator.forBlock['len'] = function(block, generator) {
  return `{ "type": "len" }`;
};

generator.forBlock['get'] = function(block, generator) {
  throw new Error('Not implemented: get');
};

generator.forBlock['set'] = function(block, generator) {
  throw new Error('Not implemented: set');
};

generator.forBlock['sleep'] = function(block, generator) {
  const delay = generator.valueToCode(block, 'delay', Order.ATOMIC);
  return `{ "type": "sleep", "delay": ${delay} }`;
};
