package helpers

import "github.com/notmalte/psce/internal/constants"

func CastlingString(castling uint8) string {
	if castling == constants.CastlingNone {
		return "-"
	}

	s := ""

	if castling&constants.CastlingWhiteKingside != 0 {
		s += "K"
	}
	if castling&constants.CastlingWhiteQueenside != 0 {
		s += "Q"
	}
	if castling&constants.CastlingBlackKingside != 0 {
		s += "k"
	}
	if castling&constants.CastlingBlackQueenside != 0 {
		s += "q"
	}

	return s
}

func StringToCastling(s string) uint8 {
	castling := constants.CastlingNone

	if s == "-" {
		return castling
	}

	for _, c := range s {
		switch c {
		case 'K':
			castling |= constants.CastlingWhiteKingside
		case 'Q':
			castling |= constants.CastlingWhiteQueenside
		case 'k':
			castling |= constants.CastlingBlackKingside
		case 'q':
			castling |= constants.CastlingBlackQueenside
		}
	}

	return castling
}
