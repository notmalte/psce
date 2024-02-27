package main

import (
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
)

func main() {
	bb := bitboard.Bitboard(0)

	bb.SetBit(constants.A4)

	fmt.Println(bb.String())
}
