package main

import (
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
)

func main() {
	bb := uint64(0)

	bitboard.SetBit(&bb, constants.A4)

	fmt.Println(bitboard.String(bb))
}
