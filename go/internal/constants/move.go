package constants

const MoveFlagNone uint8 = 0

const (
	MoveFlagPromotion uint8 = 1 << iota
	MoveFlagCapture
	MoveFlagEnPassant
	MoveFlagCastle
	MoveFlagDoublePawnPush
)
