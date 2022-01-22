Build to wasm file:

`GOOS=js GOARCH=wasm go build -o ../../assets/json.wasm`

Installation:

Copy `wasm_exec.js` to `wasm/assets`. The file can be found in golang environment. For my case it was in:

```
~⚙ ls ~/go/misc/wasm/*.js
/home/t/go/misc/wasm/wasm_exec.js
```