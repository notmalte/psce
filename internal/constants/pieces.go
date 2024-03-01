package constants

import "errors"

const (
	WhitePawn uint8 = iota
	WhiteKnight
	WhiteBishop
	WhiteRook
	WhiteQueen
	WhiteKing
	BlackPawn
	BlackKnight
	BlackBishop
	BlackRook
	BlackQueen
	BlackKing
	PiecesCount
)

func PieceString(piece uint8) string {
	switch piece {
	case WhitePawn:
		return "P"
	case WhiteKnight:
		return "N"
	case WhiteBishop:
		return "B"
	case WhiteRook:
		return "R"
	case WhiteQueen:
		return "Q"
	case WhiteKing:
		return "K"
	case BlackPawn:
		return "p"
	case BlackKnight:
		return "n"
	case BlackBishop:
		return "b"
	case BlackRook:
		return "r"
	case BlackQueen:
		return "q"
	case BlackKing:
		return "k"
	default:
		return "?"
	}
}

func PieceColor(piece uint8) uint8 {
	if piece < BlackPawn {
		return ColorWhite
	} else {
		return ColorBlack
	}
}

var ErrInvalidPiece = errors.New("invalid piece")

func StringToPiece(s string) (uint8, error) {
	switch s {
	case "P":
		return WhitePawn, nil
	case "N":
		return WhiteKnight, nil
	case "B":
		return WhiteBishop, nil
	case "R":
		return WhiteRook, nil
	case "Q":
		return WhiteQueen, nil
	case "K":
		return WhiteKing, nil
	case "p":
		return BlackPawn, nil
	case "n":
		return BlackKnight, nil
	case "b":
		return BlackBishop, nil
	case "r":
		return BlackRook, nil
	case "q":
		return BlackQueen, nil
	case "k":
		return BlackKing, nil
	default:
		return 0, ErrInvalidPiece
	}
}
