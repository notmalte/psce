package main

import (
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
)

func main() {
	bb := bitboard.Bitboard(0)

	bb.SetIndex(bitboard.A4)

	fmt.Println(bb.String())
}
