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

func (m *Move) IsPromotion() bool {
	return m.HasFlag(constants.MoveFlagPromotion)
}

func (m *Move) IsCapture() bool {
	return m.HasFlag(constants.MoveFlagCapture)
}

func (m *Move) IsEnPassant() bool {
	return m.HasFlag(constants.MoveFlagEnPassant)
}

func (m *Move) IsCastle() bool {
	return m.HasFlag(constants.MoveFlagCastle)
}

func (m *Move) IsDoublePawnPush() bool {
	return m.HasFlag(constants.MoveFlagDoublePawnPush)
}

func (m *Move) Resets50MoveRule() bool {
	return m.IsCapture() || m.Piece == constants.WhitePawn || m.Piece == constants.BlackPawn
}

func (m *Move) ResetsRepetition() bool {
	// missing: moves which lose castling rights as we don't have that information here
	return m.IsCapture() || m.IsCastle() || m.Piece == constants.WhitePawn || m.Piece == constants.BlackPawn
}

func (m *Move) String() string {
	s := fmt.Sprintf("%s%s", helpers.SquareString(m.FromSquare), helpers.SquareString(m.ToSquare))

	if m.IsPromotion() {
		s += fmt.Sprintf(" [PROMOTION: %s]", helpers.PieceString(m.PromotionPiece))
	}

	if m.IsCapture() {
		s += " [CAPTURE]"
	}

	if m.IsEnPassant() {
		s += " [EN PASSANT]"
	}

	if m.IsCastle() {
		s += " [CASTLE]"
	}

	if m.IsDoublePawnPush() {
		s += " [DOUBLE PAWN PUSH]"
	}

	return s
}

func (m *Move) UciString() string {
	s := fmt.Sprintf("%s%s", helpers.SquareString(m.FromSquare), helpers.SquareString(m.ToSquare))

	if m.HasFlag(constants.MoveFlagPromotion) {
		s += helpers.PieceString(m.PromotionPiece)
	}

	return s
}
