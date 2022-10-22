# Web assembly JSON parsers benchmark

<img width="670" alt="image" src="https://user-images.githubusercontent.com/40120335/184539061-ad5a9c1f-2b41-49e8-945a-cb4b5fbe0a0c.png">

Simple web-assembly benchmark based on custom JSON-parsers. The motivation for this project is to integrate web-assembly and present benchmark results in readable form of web application. Test data are completly custom, and they can be provided by input directly from a browser, or pre-defined test data can be used as an option.

## 🚴 installation

This project requires environmnents for: 
 - Go
 - Rust
 - nodejs/yarn
<details><summary><i>Server installation</i></summary>

### 🚴  Server installation
This project uses simple Go based server. It serves static files located in `assets`.
In order to run this server follow instructions below.
Go to the location

`cd cmd/server/`

And run

`go build`
</details> 
<details><summary><i>Frontend installation</i></summary>

### 🚴  Frontend installation

Go to the (you can use --cwd as well)

`cd cmd/server/front`

And run installation 

`yarn`
</details>
<details><summary><i>Go wasm app installation</i></summary>

### 🚴  Go wasm app installation

Go to the location

`cd cmd/wasm-go`

And follow the installation chapter in the [README.md](cmd/wasm-go/README.md) file 
</details>
<details><summary><i>Rust wasm app installation</i></summary>

### 🚴  Rust wasm app installation

Go to the location

`cd cmd/wasm-rust`

And follow the installation chapter in the [README.md](cmd/wasm-rust/README.md) file 

</details>

## 🛠️ Running project

Build frontend app using this command:

`yarn build`

As a result a new file should be created in `assets/dist/`

Now run the server:

`cd cmd/server`

`./server`

### Screenshot

<img width="1011" alt="image" src="https://user-images.githubusercontent.com/40120335/184535465-d7796ad4-2544-43dc-b196-667f37201041.png">

