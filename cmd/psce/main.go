package main

import (
	"flag"
	"github.com/notmalte/psce/internal/api"
	"github.com/notmalte/psce/internal/interactive"
)

func main() {
	apiMode := flag.Bool("api", false, "Run in API mode")
	flag.Parse()

	if *apiMode {
		api.Run()
		return
	}

	interactive.Run()
}
