package position

import (
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
)

type Position struct {
	PieceBitboards  [12]uint64
	ColorBitboards  [3]uint64
	EnPassantSquare uint8
	CastlingRights  uint8
	ColorToMove     uint8
}

func New() *Position {
	pos := &Position{}
	pos.PieceBitboards = [12]uint64{}
	pos.ColorBitboards = [3]uint64{}
	pos.EnPassantSquare = constants.NoSquare
	pos.CastlingRights = 0
	pos.ColorToMove = constants.ColorWhite

	return pos
}

func (pos *Position) String() string {
	s := ""

	for row := range uint8(8) {
		s += fmt.Sprintf("%d ", 8-row)
	outer:
		for col := range uint8(8) {
			index := bitboard.RowColToIndex(row, col)

			for piece := range constants.PiecesCount {
				if bitboard.GetBit(pos.PieceBitboards[piece], index) {
					s += fmt.Sprintf("%s ", constants.PieceString(piece))
					continue outer
				}
			}

			s += ". "
		}
		s += "\n"
	}

	s += "  A B C D E F G H\n\n"

	s += fmt.Sprintf("Color to move: %s\n", constants.ColorString(pos.ColorToMove))
	if pos.EnPassantSquare != constants.NoSquare {
		s += fmt.Sprintf("En passant square: %s\n", constants.SquareString(pos.EnPassantSquare))
	} else {
		s += "En passant square: -\n"
	}
	s += fmt.Sprintf("Castling rights: %s", constants.CastlingString(pos.CastlingRights))

	return s
}
