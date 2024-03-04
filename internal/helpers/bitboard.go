package helpers

import (
	"errors"
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
)

func SquareString(square uint8) string {
	row, col := bitboard.IndexToRowCol(square)
	return fmt.Sprintf("%c%d", 'a'+col, 8-row)
}

var ErrInvalidSquare = errors.New("invalid square")

func StringToSquare(s string) (uint8, error) {
	if len(s) != 2 {
		return constants.NoSquare, ErrInvalidSquare
	}

	colStr := s[0]
	rowStr := s[1]

	if colStr < 'a' || colStr > 'h' || rowStr < '1' || rowStr > '8' {
		return constants.NoSquare, ErrInvalidSquare
	}

	col := s[0] - 'a'
	row := '8' - s[1]

	return bitboard.RowColToIndex(row, col), nil
}

func GetMirrorSquare(square uint8) uint8 {
	return (7-(square/8))*8 + square%8
}
