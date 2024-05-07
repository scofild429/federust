# Propose
This project is going to create a federated learning framework with Rust. Known for its safety, Rust provides a terrific playground for our propose. Our propose is to design a architecture, which can combine decenterlized learning and cross-Silo learning use cases. At the same time, privacy problem and poisoning attack will also be token into consideration. This project has been divided into three modules. The communication can be implemented with REST-form between different ports OR with WebAssembly. 

![ Liberal Federated learning](./feder.png)

# Implementation
There are two different designs has been mentioned, RestAPI with mirco-service and serverless WebAssembly.

## RestAPI with mirco-service
Our three modules must be deployed in different ports. They communicate with http requests. 

## Serverless WebAssembly
The Interface module will be implemented with Vue and TS, Client module and Server module will be wrote in RUST at first, and the function from RUST will be compiled into WebAssembly and exposed to TS.

For this implementation, we will compile all the functions in client and server module into WebAssembly, and expose them to interface module with WebAssembly. In this way we can call all the funcions from interface module as serverless request.

A big question mark is, do all dependencies we need support WebAssemly?


Because of the rich ecosystem of Python for AI, we want allow our system also be able to run Python code, for training and ONNX pool aggregation. We have two options, compile Python into WebAssembly and call python from Rust with Foreign Function Interface. Which of them works and how they performance comes later.
