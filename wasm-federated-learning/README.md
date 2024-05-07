# project target
This project is trying to combine client and server module in one rust project, and offer all their function through WebAssembly, and import the WebAssembly into interface module as dependenices, like npm package, so we can call those function with TS/JS in interface module. That is a serverless design.

# How it works
generate package,
```sh 
cargo generate --git https://github.com/rustwasm/wasm-pack-template
wasm-pack build 
```
This will create wasm package which is stored in pkg directory, and import this wasm package as dependencies in npm project. So we can call the exposed functions.

