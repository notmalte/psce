package main

import (
	"fmt"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/position"
)

func main() {
	pos, err := position.PositionFromFen(constants.InitialPositionFEN)
	if err != nil {
		panic(err)
	}

	fmt.Println(pos)
}
