# enigma-stats

Simple web-assembly benchmark based on custom JSON-parsers. The motivation for this project is to integrate web-assembly and present benchmark results in readable form of web application. Test data are completly custom, and they can be provided by input directly from a browser, or pre-defined test data can be used as an option.

## 🚴 installation

This project requires environmnents for: 
 - Go
 - Rust
 - nodejs/yarn

### 🚴  server installation
This project uses simple Go based server. It serves static files located in `assets`.
In order to run this server follow instructions below.
Go to the location

`cd cmd/server/`

And run

`go build`
 
### 🚴  frontend installation

Go to the (you can use --cwd as well)

`cd cmd/server/front`

And run installation 

`yarn`


### 🚴  Go wasm app installation

Go to the location

`cd cmd/wasm-go`

And follow the installation chapter in the [README.md](cmd/wasm-go/README.md) file 

### 🚴  Rust wasm app installation

Go to the location

`cd cmd/wasm-rust`

And follow the installation chapter in the [README.md](cmd/wasm-rust/README.md) file 

## 🛠️ Running project

Build frontend app using this command:

`yarn build`

As a result a new file should be created in `assets/dist/`

Now run the server:

`cd cmd/server`

`./server`

### Screenshot

![image](https://user-images.githubusercontent.com/40120335/152691314-9325925c-e6b3-4f11-ba39-002c9150571b.png)
