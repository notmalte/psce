package constants

const CastlingNone uint8 = 0

const (
	CastlingWhiteKingside uint8 = 1 << iota
	CastlingWhiteQueenside
	CastlingBlackKingside
	CastlingBlackQueenside
)

const (
	CastlingWhiteKingsideEmptyMask  uint64 = (1 << F1) | (1 << G1)
	CastlingWhiteQueensideEmptyMask        = (1 << B1) | (1 << C1) | (1 << D1)
	CastlingBlackKingsideEmptyMask         = (1 << F8) | (1 << G8)
	CastlingBlackQueensideEmptyMask        = (1 << B8) | (1 << C8) | (1 << D8)
)
