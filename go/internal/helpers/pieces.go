package helpers

import (
	"errors"
	"github.com/notmalte/psce/internal/constants"
)

func PieceString(piece uint8) string {
	switch piece {
	case constants.WhitePawn:
		return "P"
	case constants.WhiteKnight:
		return "N"
	case constants.WhiteBishop:
		return "B"
	case constants.WhiteRook:
		return "R"
	case constants.WhiteQueen:
		return "Q"
	case constants.WhiteKing:
		return "K"
	case constants.BlackPawn:
		return "p"
	case constants.BlackKnight:
		return "n"
	case constants.BlackBishop:
		return "b"
	case constants.BlackRook:
		return "r"
	case constants.BlackQueen:
		return "q"
	case constants.BlackKing:
		return "k"
	default:
		return "?"
	}
}

func PieceStringUnicode(piece uint8) string {
	switch piece {
	case constants.WhitePawn:
		return "♙"
	case constants.WhiteKnight:
		return "♘"
	case constants.WhiteBishop:
		return "♗"
	case constants.WhiteRook:
		return "♖"
	case constants.WhiteQueen:
		return "♕"
	case constants.WhiteKing:
		return "♔"
	case constants.BlackPawn:
		return "♟"
	case constants.BlackKnight:
		return "♞"
	case constants.BlackBishop:
		return "♝"
	case constants.BlackRook:
		return "♜"
	case constants.BlackQueen:
		return "♛"
	case constants.BlackKing:
		return "♚"
	default:
		return "?"
	}
}

func PieceColor(piece uint8) uint8 {
	if piece < constants.BlackPawn {
		return constants.ColorWhite
	} else {
		return constants.ColorBlack
	}
}

var ErrInvalidPiece = errors.New("invalid piece")

func StringToPiece(s string) (uint8, error) {
	switch s {
	case "P":
		return constants.WhitePawn, nil
	case "N":
		return constants.WhiteKnight, nil
	case "B":
		return constants.WhiteBishop, nil
	case "R":
		return constants.WhiteRook, nil
	case "Q":
		return constants.WhiteQueen, nil
	case "K":
		return constants.WhiteKing, nil
	case "p":
		return constants.BlackPawn, nil
	case "n":
		return constants.BlackKnight, nil
	case "b":
		return constants.BlackBishop, nil
	case "r":
		return constants.BlackRook, nil
	case "q":
		return constants.BlackQueen, nil
	case "k":
		return constants.BlackKing, nil
	default:
		return 0, ErrInvalidPiece
	}
}
