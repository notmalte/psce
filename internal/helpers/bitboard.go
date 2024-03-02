package helpers

import (
	"errors"
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
)

func SquareString(square uint8) string {
	row, col := bitboard.IndexToRowCol(square)
	return fmt.Sprintf("%c%d", 'A'+col, 8-row)
}

var ErrInvalidSquare = errors.New("invalid square")

func StringToSquare(s string) (uint8, error) {
	if len(s) != 2 {
		return constants.NoSquare, ErrInvalidSquare
	}

	col := s[0] - 'A'
	row := 8 - (s[1] - '0')

	if col < 0 || col > 7 || row < 0 || row > 7 {
		return constants.NoSquare, ErrInvalidSquare
	}

	return bitboard.RowColToIndex(row, col), nil
}
