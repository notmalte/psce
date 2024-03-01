package constants

const (
	CastlingNone           uint8 = 0b0000
	CastlingWhiteKingside  uint8 = 0b0001
	CastlingWhiteQueenside uint8 = 0b0010
	CastlingBlackKingside  uint8 = 0b0100
	CastlingBlackQueenside uint8 = 0b1000
)

func CastlingString(castling uint8) string {
	if castling == CastlingNone {
		return "-"
	}

	s := ""

	if castling&CastlingWhiteKingside != 0 {
		s += "K"
	}
	if castling&CastlingWhiteQueenside != 0 {
		s += "Q"
	}
	if castling&CastlingBlackKingside != 0 {
		s += "k"
	}
	if castling&CastlingBlackQueenside != 0 {
		s += "q"
	}

	return s
}
