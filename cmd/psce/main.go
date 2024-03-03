package main

import (
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/perft"
	"github.com/notmalte/psce/internal/position"
)

func main() {
	pos := position.Initial()

	mg := movegen.NewMoveGen()

	for depth := uint(1); depth <= 5; depth++ {
		perft.RunPerft(mg, pos, depth)
	}
}
