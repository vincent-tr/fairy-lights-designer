# fairy-lights-designer

## Setup environment

```bash
rustup update
rustup target add wasm32-unknown-unknown
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Notes

- Interpreter :
  - Interpreter : https://github.com/hashemi/cloxpp

```
Le programme doit être infini avec boucles et sleep

Opcodes sur 8 bytes + 24 bytes arguments

4 fonctions :
- sleep(ms)
- (r, g, b) = get(index)
- set(index, r, g, b)
- res = len() // longueur de la guirlande 

Variables locales de type i32
(Autant qu'on veut, initialisé a 0, le compiler traduit nom => index, et le runtime alloue les index nouveau a la volée)

If/loop/break/continue
=> Label, jumps conditionnels si 0 ou si !0, jump inconditionnels
+ - * / %
Literals (i24 ?)
Op < > != == ! Qui remplit un i32 a 1 ou 0
```

- Blockly :
  - Blockly custom generator : https://github.com/google/blockly-samples/tree/master/codelabs/custom_generator
  - Blockly example : https://github.com/jaelle/blockly-page-editor/blob/gh-pages/blockly.html
  - Blockly developer tools : https://google.github.io/blockly-samples/examples/developer-tools/index.html

- Wasm :
  - Rust wasm : https://rustwasm.github.io/wasm-bindgen/examples/julia.html
  - Wasm engine : https://rustwasm.github.io/docs/book/game-of-life/implementing.html
