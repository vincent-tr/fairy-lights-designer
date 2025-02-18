import * as Blockly from 'blockly';

const Order = {
  ATOMIC: 0,
};

export const generator = new Blockly.Generator('fairy-lights-runtime');

generator.forBlock['logic_compare'] = function(block, generator) {
  
  const OPERATORS = {
    'EQ': 'eq',
    'NEQ': 'neq',
    'LT': 'lt',
    'LTE': 'lte',
    'GT': 'gt',
    'GTE': 'gte',
  };

  return operator_ab(block, generator, OPERATORS);
};

generator.forBlock['logic_operation'] = function(block, generator) {
  
  const OPERATORS = {
    'AND': 'and',
    'OR': 'or',
  }

  return operator_ab(block, generator, OPERATORS);
}

generator.forBlock['logic_negate'] = function(block, generator) {
  const op1 = generator.valueToCode(block, 'BOOL', order);

  if (!op1) {
    throw new Error('Missing operand');
  }

  return [
    `{ "type": "not", "value": ${op1} }`,
    Order.ATOMIC
  ];
}

generator.forBlock['logic_boolean'] = function(block, generator) {
  const value = block.getFieldValue('BOOL') === 'TRUE';

  return [
    `{ "type": "literal_boolean", "value": ${value} }`,
    Order.ATOMIC
  ];
}

generator.forBlock['controls_if'] = function(block, generator) {
  const branches = [];

  for (let n = 0; block.getInput('IF' + n); ++n) {
    const cond = generator.valueToCode(block, 'IF' + n, Order.ATOMIC);
    const body = generator.statementToCode(block, 'DO' + n);

    if (!cond || !body) {
      throw new Error('Missing operands');
    }

    branches.push(`{ "condition": ${cond}, "body": ${body} }`);
  }

  if (branches.length === 0) {
    throw new Error('Missing branches');
  }

  if (block.getInput('ELSE')) {
    const body = generator.statementToCode(block, 'ELSE');
    branches.push(`{ "condition": null, "body": ${body} }`);
  }

  return `{ "type": "if", "branches": [${branches.join(', ')}] }`;
};

generator.forBlock['controls_repeat_ext'] = function(block, generator) {

  const repeats = generator.valueToCode(block, 'TIMES', Order.ATOMIC);
  const branch = generator.statementToCode(block, 'DO');

  if (!repeats || !branch) {
    throw new Error('Missing operands');
  }

  return `{ "type": "repeat", "times": ${repeats}, "body": ${branch} }`;
}

generator.forBlock['controls_whileUntil'] = function(block, generator) {
  const TYPES = {
    'UNTIL': 'until',
    'WHILE': 'while',
  }

  const type = TYPES[block.getFieldValue('MODE')];
  const cond = generator.valueToCode(block, 'BOOL', Order.ATOMIC);
  const body = generator.statementToCode(block, 'DO');

  if (!op) {
    throw new Error('Unknown type: ' + block.getFieldValue('MODE'));
  }

  if (!cond || !body) {
    throw new Error('Missing operands');
  }

  return `{ "type": "${type}", "condition": ${cond}, "body": ${body} }`;
}

generator.forBlock['controls_for'] = function(block, generator) {

  const variable = block.getFieldValue('VAR');
  const argFrom = generator.valueToCode(block, 'FROM', Order.ASSIGNMENT);
  const argTo = generator.valueToCode(block, 'TO', Order.ASSIGNMENT);
  const argBy = generator.valueToCode(block, 'BY', Order.ASSIGNMENT);
  const body = generator.statementToCode(block, 'DO');

  if (!variable || !argFrom || !argTo || !argBy || !body) {
    throw new Error('Missing operands');
  }

  return `{ "type": "for", "variable": "${variable}", "from": ${argFrom}, "to": ${argTo}, "by": ${argBy}, "body": ${body} }`;
}

generator.forBlock['controls_flow_statements'] = function(block, generator) {
  const TYPES = {
    'BREAK': 'break',
    'CONTINUE': 'continue',
  }

  const type = TYPES[block.getFieldValue('FLOW')];

  if (!type) {
    throw new Error('Unknown type: ' + block.getFieldValue('FLOW'));
  }

  return `{ "type": "${type}" }`;
}

generator.forBlock['math_number'] = function(block, generator) {
  const value = block.getFieldValue('NUM');
  
  return [
    `{ "type": "literal", "value": ${value} }`,
    Order.ATOMIC
  ];
}

generator.forBlock['math_arithmetic'] = function(block, generator) {

  const OPERATORS = {
    'ADD': 'add',
    'MINUS': 'sub',
    'MULTIPLY': 'mul',
    'DIVIDE': 'div',
    'POWER': 'pow',
  };

  return operator_ab(block, generator, OPERATORS);
}

generator.forBlock['math_modulo'] = function(block, generator) {
  const op1 = generator.valueToCode(block, 'A', Order.ATOMIC);
  const op2 = generator.valueToCode(block, 'B', Order.ATOMIC);

  if (!op1 || !op2) {
    throw new Error('Missing operands');
  }

  return [
    `{ "type": "mod", "op1": ${op1}, "op2": ${op2} }`,
    Order.ATOMIC
  ];
}

generator.forBlock['math_constrain'] = function(block, generator) {
  const value = generator.valueToCode(block, 'VALUE', Order.ATOMIC);
  const low = generator.valueToCode(block, 'LOW', Order.ATOMIC);
  const high = generator.valueToCode(block, 'HIGH', Order.ATOMIC);

  if (!value || !low || !high) {
    throw new Error('Missing operands');
  }

  return [
    `{ "type": "between", "value": ${value}, "low": ${low}, "high": ${high} }`,
    Order.ATOMIC
  ];
}

generator.forBlock['math_random_int'] = function(block, generator) {
  const min = generator.valueToCode(block, 'FROM', Order.ATOMIC);
  const max = generator.valueToCode(block, 'TO', Order.ATOMIC);

  if (!min || !max) {
    throw new Error('Missing operands');
  }

  return [
    `{ "type": "rand", "min": ${min}, "max": ${max} }`,
    Order.ATOMIC
  ];
}

generator.forBlock['variables_get'] = function(block, generator) {
  const id = block.getFieldValue('VAR');
  return [
    `{ "type": "get_variable", "variable": "${id}" }`,
    Order.ATOMIC
  ];
}

generator.forBlock['variables_set'] = function(block, generator) {
  const name = block.getFieldValue('VAR');
  const value = generator.valueToCode(block, 'VALUE', Order.ATOMIC);
  return `{ "type": "set_variable", "variable": "${name}", "value": ${value} }`;
}

generator.forBlock['len'] = function(block, generator) {
  return [
    `{ "type": "len" }`,
    Order.ATOMIC
  ];
};

generator.forBlock['get'] = function(block, generator) {
  const TYPES = {
    'r': 'red',
    'g': 'green',
    'b': 'blue',
  };

  const index = generator.valueToCode(block, 'index', Order.ATOMIC);
  const type = TYPES[block.getFieldValue('type')];

  if (!index || !type) {
    throw new Error('Missing operands');
  }

  return [
    `{ "type": "get", "index": ${index}, "type": "${type}" }`,
    Order.ATOMIC
  ];
};

generator.forBlock['set'] = function(block, generator) {
  const index = generator.valueToCode(block, 'index', Order.ATOMIC);
  const red = generator.valueToCode(block, 'red', Order.ATOMIC);
  const green = generator.valueToCode(block, 'green', Order.ATOMIC);
  const blue = generator.valueToCode(block, 'blue', Order.ATOMIC);

  
  return `{ "type": "set", "index": ${index}, "red": ${red}, "green": ${green}, "blue": ${blue} }`;
};

generator.forBlock['sleep'] = function(block, generator) {
  const delay = generator.valueToCode(block, 'delay', Order.ATOMIC);
  return `{ "type": "sleep", "delay": ${delay} }`;
};

function operator_ab(block, generator, operators) {
  const op = operators[block.getFieldValue('OP')];
  const op1 = generator.valueToCode(block, 'A', Order.ATOMIC);
  const op2 = generator.valueToCode(block, 'B', Order.ATOMIC);

  if (!op) {
    throw new Error('Unknown operator: ' + block.getFieldValue('OP'));
  }

  if (!op1 || !op2) {
    throw new Error('Missing operands');
  }

  return [
    `{ "type": "${op}", "op1": ${op1}, "op2": ${op2} }`,
    Order.ATOMIC
  ];
}
