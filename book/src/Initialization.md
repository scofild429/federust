# Project initialization

## Book for development
``` sh
mdbook init book
mdbook build
```
	for development manual and API implementation

## Interface module 
``` sh
npm create vite@latest
```
user interface implementation with bootstrap. Wasm libraries will be imported as dependencies and be called with TS.

## Database
we use sqlx-cli for migration. .env will be automatically loaded for sqlx-cli commands.
```sh
sqlx create database
sqlx migrate add -r user
sqlx migrate run 
sqlx migrate revert
```


## Liberal-federated-learning
 ``` sh
cargo insatall wasm-pack
cargo insatall cargo-generate
cargo generate --git https://github.com/rustwasm/wasm-pack-template
wasm-pack build
```
call client and server API within workspace, build to WebAssembly for interface.


### Client-module
``` sh
cargo new client-module
```
client module API implementation

### Server-module
``` sh
cargo new server-module
```
client module API implementation

