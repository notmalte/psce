package main

import (
	"fmt"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
)

func main() {
	pos := position.Initial()
	fmt.Println(pos)

	mg := movegen.NewMoveGen()

	moves := mg.GeneratePseudoLegalMoves(pos)
	for _, move := range moves {
		fmt.Println(&move)
	}
}
