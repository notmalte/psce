package constants

const CastlingNone uint8 = 0

const (
	CastlingWhiteKingside uint8 = 1 << iota
	CastlingWhiteQueenside
	CastlingBlackKingside
	CastlingBlackQueenside
)
