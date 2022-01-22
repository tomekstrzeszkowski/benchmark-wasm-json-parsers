package main

import (
	"fmt"
	"github.com/TwiN/go-color"
)

func main() {
	fmt.Println(fmt.Sprintf("Hello%s webassembly!%s", color.Red, color.Reset))
}