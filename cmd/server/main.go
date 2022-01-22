package main

import (
	"fmt"
	"net/http"
	"github.com/TwiN/go-color"
)

func main() {
	var port uint16 = 9090
	formattedPort := fmt.Sprintf(":%d", port)
	fmt.Println(fmt.Sprintf("Serving files on port -> %s%d%s", color.Red, port, color.Reset))
	err := http.ListenAndServe(formattedPort, http.FileServer(http.Dir("../../assets")))
	if err != nil {
		fmt.Println("Failed to start server", err)
		return
	}
}