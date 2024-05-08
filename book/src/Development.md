# Development Records

## Tue May  7 20:02:26 2024
- RestAPI or WebAssembly 

As the fundamental question of this project, we have to decide that how to communicate between different module, i.e., interface, client and server. Using Restapi with http request or Serverless Webassembly for function calling. Now we are exploring Webassembly, when Client module send data to postgressql for register and authentication, we discovered that Webassembly does NOT support sending postgressql request yet.

More detail, in our case, we want compose a function in client module, which will connect to postgressql and insert data into table. The same code work in pure rust project, but in our project, those code will be compiled into WebAssembly with wasm-pack. The following is the compiler error.
```
error[E0432]: unresolved import `crate::sys::IoSourceState`
```
There are totally 44 package that can not be refered to wasm-pack. So we have to give up WebAssembly.


## Wed May  8 11:13:53 2024
- Pure rust project with template engine

Before we start Rest design(separeted fronted and backend project), I want to start the third design. All moudles are included in one single rust project, interface module is rendered by template engine, client and server module can be designed as sub-mod. Now I am going to try Tera template engine, see if this design works.
