package main

import (
	"fmt"

	"github.com/TwiN/go-color"

	"syscall/js"

	"webassembly.com/web/car"
)

func goParseJson(this js.Value, args []js.Value) interface{} {
	return car.ParseJSON(args[0].String())
}

func main() {
	c := make(chan struct{})
	text := fmt.Sprintf("Hello%s webassembly!%s", color.Red, color.Reset)
	fmt.Println(text)
	fmt.Println("Try this example:")
	fmt.Println("parseJson('[{\"Name\":\"chevy s-10\",\"Miles_per_Gallon\":31,\"Cylinders\":4,\"Displacement\":\"119\",\"Horsepower\":82,\"Weight_in_lbs\":2720,\"Acceleration\":19.4,\"Year\":\"1982-01-01\",\"Origin\":\"USA\"}]')")
	js.Global().Set("parseJson", js.FuncOf(goParseJson))
	<-c
}
