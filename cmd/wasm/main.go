package main

import (
	"fmt"

	"github.com/TwiN/go-color"

	"webassembly.com/web/animal"
	"webassembly.com/web/car"
)

func main() {
	// car := car.Car{"a"}
	// fmt.Println(car.Name)
	text := fmt.Sprintf("Hello%s webassembly!%s", color.Red, color.Reset)
	fmt.Println(text)
	animal.Main()
	car.MakeCars()
}

//to each his own