# Project initialization

- book
    ``` sh
    mdbook init book
    mdbook build
    ```
    for development manual and API implementation
- interface
    ``` sh
    npm create vite@latest
    ```
    user interface implementation with bootstrap. Wasm libraries will be imported as dependencies and be called with TS.


- liberal-federated-learning
     ``` sh
    cargo insatall wasm-pack
    cargo insatall cargo-generate
    cargo generate --git https://github.com/rustwasm/wasm-pack-template
    wasm-pack build
    ```
    call client and server API within workspace, build to WebAssembly for interface.


  - client-module
    ``` sh
    cargo new client-module
    ```
    client module API implementation

  - server-module
    ``` sh
    cargo new server-module
    ```
    client module API implementation

