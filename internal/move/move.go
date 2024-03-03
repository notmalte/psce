package move

import (
	"fmt"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/helpers"
)

type Move struct {
	FromSquare     uint8
	ToSquare       uint8
	Piece          uint8
	Flags          uint8
	PromotionPiece uint8
}

func (m *Move) HasFlag(flag uint8) bool {
	return m.Flags&flag != 0
}

func (m *Move) String() string {
	s := fmt.Sprintf("%s%s", helpers.SquareString(m.FromSquare), helpers.SquareString(m.ToSquare))

	if m.HasFlag(constants.MoveFlagPromotion) {
		s += fmt.Sprintf(" [PROMOTION: %s]", helpers.PieceString(m.PromotionPiece))
	}

	if m.HasFlag(constants.MoveFlagCapture) {
		s += " [CAPTURE]"
	}

	if m.HasFlag(constants.MoveFlagEnPassant) {
		s += " [EN PASSANT]"
	}

	if m.HasFlag(constants.MoveFlagCastle) {
		s += " [CASTLE]"
	}

	if m.HasFlag(constants.MoveFlagDoublePawnPush) {
		s += " [DOUBLE PAWN PUSH]"
	}

	return s
}

func (m *Move) UciString() string {
	s := fmt.Sprintf("%s%s", helpers.SquareString(m.FromSquare), helpers.SquareString(m.ToSquare))

	if m.HasFlag(constants.MoveFlagPromotion) {
		s += fmt.Sprintf("%s", helpers.PieceString(m.PromotionPiece))
	}

	return s
}
