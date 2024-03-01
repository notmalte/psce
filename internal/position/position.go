package position

import (
	"errors"
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"strings"
)

type Position struct {
	PieceBitboards  [12]uint64
	ColorBitboards  [3]uint64
	EnPassantSquare uint8
	CastlingRights  uint8
	ColorToMove     uint8
}

func Empty() *Position {
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

var ErrInvalidFen = errors.New("invalid FEN")

func PositionFromFen(fen string) (*Position, error) {
	pos := Empty()

	fenParts := strings.Split(fen, " ")
	if len(fenParts) < 4 {
		return nil, ErrInvalidFen
	}

	rows := strings.Split(fenParts[0], "/")
	if len(rows) != 8 {
		return nil, ErrInvalidFen
	}

	for row, rowString := range rows {
		col := 0

		for _, c := range rowString {
			if c >= '1' && c <= '8' {
				col += int(c - '0')
			} else {
				index := bitboard.RowColToIndex(uint8(row), uint8(col))
				piece, err := constants.StringToPiece(string(c))
				if err != nil {
					return nil, ErrInvalidFen
				}
				color := constants.PieceColor(piece)

				bitboard.SetBit(&pos.PieceBitboards[piece], index)
				bitboard.SetBit(&pos.ColorBitboards[color], index)

				col++
			}
		}

		if col != 8 {
			return nil, ErrInvalidFen
		}
	}

	pos.ColorBitboards[constants.ColorBoth] = pos.ColorBitboards[constants.ColorWhite] | pos.ColorBitboards[constants.ColorBlack]

	if fenParts[1] == "w" {
		pos.ColorToMove = constants.ColorWhite
	} else if fenParts[1] == "b" {
		pos.ColorToMove = constants.ColorBlack
	} else {
		return nil, ErrInvalidFen
	}

	pos.CastlingRights = constants.StringToCastling(fenParts[2])

	if fenParts[3] != "-" {
		square, err := constants.StringToSquare(fenParts[3])
		if err != nil {
			return nil, ErrInvalidFen
		}
		pos.EnPassantSquare = square
	}

	return pos, nil
}
