```
componentize-py -d ./wit/world.wit -w plugin bindings plugin_guest
componentize-py -d ./wit/world.wit -w plugin componentize --stub-wasi obfuscate -o obfuscate.wasm
wasm-tools strip ./obfuscate.wasm -o obfuscate.wasm
```
