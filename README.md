# Project descripation
We are going to design and implement a federated learning framework, which is flexibel for many use cases. Please refer to [Github page](https://scofild429.github.io/federust/) for more development details. In our repository, each folder is design to be a project.

# Project design
We have three different option. 

## Rest design.
This is a separated frontend and backend project. The frontend is for interface module, from here we send HTTP requests to backend, which implemented client and server module. But we have to deploy the frontend and backend separatelly in different ports.
- frontend project: interface
- backend project: rest-federated-learning

## WebAssembly design
All client and server module functions will be compiled to Webassembly, as imported as dependence for interface module. We only need to deploy the interface module as node project.
- frontend project: interface
- Webassembly dependence project: wasm-federated-learning

## Pure Rust design
All three module will be implement with rust and template engine. Deployment is as easy as to lanuch a compiled binary file.
- pure rust project: app-federated-learning
