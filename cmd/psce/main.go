package main

import (
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
)

func main() {
	pos := position.Initial()
	fmt.Println(pos)

	mg := movegen.NewMoveGen()

	att := uint64(0)
	for square := range uint8(64) {
		if mg.IsSquareAttacked(pos, square, constants.ColorWhite) {
			att |= 1 << square
		}
	}

	fmt.Printf("\n%s\n", bitboard.String(att))
}
