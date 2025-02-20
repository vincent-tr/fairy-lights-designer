import * as Blockly from 'blockly';

const Order = {
  ATOMIC: 0,
};

class AstGenerator extends Blockly.CodeGenerator {
  constructor(name = 'fairy-lights-runtime') {
    super(name);
  }

  init(workspace) {
    super.init(workspace);

    if (!this.nameDB_) {
      this.nameDB_ = new Blockly.Names();
    } else {
      this.nameDB_.reset();
    }

    this.nameDB_.setVariableMap(workspace.getVariableMap());
    this.nameDB_.populateVariables(workspace);
  }

  scrubNakedValue(value) {
    return JSON.stringify({ type: 'naked', value: JSON.parse(value) });
  }

  scrub_(block, code, thisOnly = false) {
    const nextBlock = block.nextConnection && block.nextConnection.targetBlock();
    if (!nextBlock || thisOnly) {
      return code;
    }

    // Flatten sequences
    const next = JSON.parse(this.blockToCode(nextBlock));

    code = {
      type: 'sequence',
      items: [JSON.parse(code)],
    };

    if (next.type === 'sequence') {
      code.items.push(...next.items);
    } else {
      code.items.push(next);
    }

    return JSON.stringify(code);
  }

  objValueToCode(block, field) {
    const raw = this.valueToCode(block, field, Order.ATOMIC);
    if (!raw) {
      throw new Error('Missing operand');
    }

    return JSON.parse(raw);
  }

  objStatementToCode(block, field) {
    const raw = this.statementToCode(block, field);
    if (!raw) {
      throw new Error('Missing statement');
    }

    return JSON.parse(raw);
  }
}

export const generator = new AstGenerator();

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
  const value = generator.objValueToCode(block, 'BOOL');

  return [
    JSON.stringify({ type: 'not', value }),
    Order.ATOMIC
  ];
}

generator.forBlock['logic_boolean'] = function(block, generator) {
  const value = block.getFieldValue('BOOL') === 'TRUE';

  return [
    JSON.stringify({ type: 'literal_boolean', value }),
    Order.ATOMIC
  ];
}

generator.forBlock['controls_if'] = function(block, generator) {
  const branches = [];

  for (let n = 0; block.getInput('IF' + n); ++n) {
    const condition = generator.objValueToCode(block, 'IF' + n);
    const body = generator.objStatementToCode(block, 'DO' + n);

    branches.push({ condition, body });
  }

  if (branches.length === 0) {
    throw new Error('Missing branches');
  }

  if (block.getInput('ELSE')) {
    const body = generator.objStatementToCode(block, 'ELSE');
    branches.push({ condition: null, body });
  }

  return JSON.stringify({ type: 'if', branches });
};

generator.forBlock['controls_repeat_ext'] = function(block, generator) {

  const times = generator.objValueToCode(block, 'TIMES');
  const body = generator.objStatementToCode(block, 'DO');

  return JSON.stringify({ type: 'repeat', times, body });
}

generator.forBlock['controls_whileUntil'] = function(block, generator) {
  const TYPES = {
    'UNTIL': 'until',
    'WHILE': 'while',
  }

  const type = TYPES[block.getFieldValue('MODE')];
  const condition = generator.objValueToCode(block, 'BOOL');
  const body = generator.objStatementToCode(block, 'DO');

  if (!type) {
    throw new Error('Unknown type: ' + block.getFieldValue('MODE'));
  }

  return JSON.stringify({ type, condition, body });
}

generator.forBlock['controls_for'] = function(block, generator) {

  const variable = generator.getVariableName(block.getFieldValue('VAR'));
  const from = generator.objValueToCode(block, 'FROM');
  const to = generator.objValueToCode(block, 'TO');
  const by = generator.objValueToCode(block, 'BY');
  const body = generator.objStatementToCode(block, 'DO');

  if (!variable) {
    throw new Error('Missing variable');
  }

  return JSON.stringify({ type: 'for', variable, from, to, by, body });
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

  return JSON.stringify({ type });
}

generator.forBlock['math_number'] = function(block, generator) {
  const value = block.getFieldValue('NUM');
  
  return [
    JSON.stringify({ type: 'literal', value }),
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
  const op1 = generator.objValueToCode(block, 'A');
  const op2 = generator.objValueToCode(block, 'B');

  return [
    JSON.stringify({ type: 'mod', op1, op2 }),
    Order.ATOMIC
  ];
}

generator.forBlock['math_constrain'] = function(block, generator) {
  const value = generator.objValueToCode(block, 'VALUE');
  const low = generator.objValueToCode(block, 'LOW');
  const high = generator.objValueToCode(block, 'HIGH');

  return [
    JSON.stringify({ type: 'between', value, low, high }),
    Order.ATOMIC
  ];
}

generator.forBlock['math_random_int'] = function(block, generator) {
  const min = generator.objValueToCode(block, 'FROM');
  const max = generator.objValueToCode(block, 'TO');

  return [
    JSON.stringify({ type: 'rand', min, max }),
    Order.ATOMIC
  ];
}

generator.forBlock['variables_get'] = function(block, generator) {
  const variable = generator.getVariableName(block.getFieldValue('VAR'));

  return [
    JSON.stringify({ type: 'get_variable', variable }),
    Order.ATOMIC
  ];
}

generator.forBlock['variables_set'] = function(block, generator) {
  const variable = generator.getVariableName(block.getFieldValue('VAR'));
  const value = generator.objValueToCode(block, 'VALUE');

  return JSON.stringify({ type: 'set_variable', variable, value });
}

generator.forBlock['len'] = function(block, generator) {
  return [
    JSON.stringify({ type: 'len' }),
    Order.ATOMIC
  ];
};

generator.forBlock['get'] = function(block, generator) {
  const TYPES = {
    'r': 'red',
    'g': 'green',
    'b': 'blue',
  };

  const index = generator.objValueToCode(block, 'index', Order.ATOMIC);
  const type = TYPES[block.getFieldValue('type')];

  if (!type) {
    throw new Error('Missing operand');
  }

  return [
    JSON.stringify({ type: 'get', index, type }),
    Order.ATOMIC
  ];
};

generator.forBlock['set'] = function(block, generator) {
  const index = generator.objValueToCode(block, 'index');
  const red = generator.objValueToCode(block, 'r');
  const green = generator.objValueToCode(block, 'g');
  const blue = generator.objValueToCode(block, 'b');
  
  return JSON.stringify({ type: 'set', index, red, green, blue });
};

generator.forBlock['sleep'] = function(block, generator) {
  const delay = generator.objValueToCode(block, 'delay');

  return JSON.stringify({ type: 'sleep', delay });
};

function operator_ab(block, generator, operators) {
  const op = operators[block.getFieldValue('OP')];
  const op1 = generator.objValueToCode(block, 'A');
  const op2 = generator.objValueToCode(block, 'B');

  if (!op) {
    throw new Error('Unknown operator: ' + block.getFieldValue('OP'));
  }

  return [
    JSON.stringify({ type: op, op1, op2 }),
    Order.ATOMIC
  ];
}
