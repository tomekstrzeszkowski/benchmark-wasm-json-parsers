## 🚴 Installation

Install newest Go-lang. Last stable version for this project:
`go version go1.18.3 darwin/amd64`

## 🛠️ Build

Build to wasm file:

`GOOS=js GOARCH=wasm go build -o ../../assets/json.wasm`

Installation:

Copy `wasm_exec.js` to `wasm/assets`. 

## Prelude
This Go integration uses `wasm_exec.js` file.
The file can be found in golang environment. In my case it was:

```
~⚙ ls ~/go/misc/wasm/*.js
/home/t/go/misc/wasm/wasm_exec.js
```

## Configuration

Extra settings for VS Code:

To `.vscode/settings.json` add this entry:

```
{
    "go.toolsEnvVars": {
        "GOOS": "js",
        "GOARCH": "wasm"
    }
}
```
