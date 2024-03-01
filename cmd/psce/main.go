package main

import (
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/position"
)

func main() {
	pos := position.New()
	bitboard.SetBit(&pos.PieceBitboards[constants.WhitePawn], constants.A3)
	pos.ColorToMove = constants.ColorBlack
	pos.EnPassantSquare = constants.A2
	pos.CastlingRights = constants.CastlingWhiteKingside | constants.CastlingWhiteQueenside | constants.CastlingBlackKingside | constants.CastlingBlackQueenside
	fmt.Println(pos)
}
