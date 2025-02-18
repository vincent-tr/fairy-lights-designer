export default [
  {
    "type": "len",
    "tooltip": "Length of the light strip",
    "helpUrl": "",
    "message0": "Length",
    "args0": [],
    "output": "Number",
    "colour": 285,
  },
  {
    "type": "get",
    "tooltip": "Get the color of a light",
    "helpUrl": "",
    "message0": "Get light color\nIndex = %1\nType = %2",
    "args0": [
      {
        "type": "input_value",
        "name": "index",
        "check": "Number"
      },
      {
        "type": "field_dropdown",
        "name": "type",
        "options": [
          [
            "Red",
            "r"
          ],
          [
            "Green",
            "g"
          ],
          [
            "Blue",
            "b"
          ]
        ]
      }
    ],
    "output": "Number",
    "colour": 285
  },
  {
    "type": "set",
    "tooltip": "Set the color of a light",
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
    "colour": 285
  },
  {
    "type": "sleep",
    "tooltip": "Sleep for a number of milliseconds",
    "helpUrl": "",
    "message0": "Sleep\nDelay = %1ms",
    "args0": [
      {
        "type": "input_value",
        "name": "delay",
        "check": "Number"
      }
    ],
    "previousStatement": null,
    "nextStatement": null,
    "colour": 285
  }
];