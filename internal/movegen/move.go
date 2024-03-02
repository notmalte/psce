package movegen

import (
	"fmt"
	"github.com/notmalte/psce/internal/helpers"
)

type Move struct {
	FromSquare     uint8
	ToSquare       uint8
	Piece          uint8
	Flags          uint8
	PromotionPiece uint8
}

const (
	FlagPromotion uint8 = 1 << iota
	FlagCapture
	FlagEnPassant
	FlagCastle
	FlagDoublePawnPush
)

func (m *Move) HasFlag(flag uint8) bool {
	return m.Flags&flag != 0
}

func (m *Move) String() string {
	s := fmt.Sprintf("%s%s", helpers.SquareString(m.FromSquare), helpers.SquareString(m.ToSquare))

	if m.HasFlag(FlagPromotion) {
		s += fmt.Sprintf(" [PROMOTION: %s]", helpers.PieceString(m.PromotionPiece))
	}

	if m.HasFlag(FlagCapture) {
		s += " [CAPTURE]"
	}

	if m.HasFlag(FlagEnPassant) {
		s += " [EN PASSANT]"
	}

	if m.HasFlag(FlagCastle) {
		s += " [CASTLE]"
	}

	if m.HasFlag(FlagDoublePawnPush) {
		s += " [DOUBLE PAWN PUSH]"
	}

	return s
}
