package movegen

import (
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/position"
)

type MoveGen struct {
	PawnMoveGen   *PawnMoveGen
	KnightMoveGen *KnightMoveGen
	BishopMoveGen *BishopMoveGen
	RookMoveGen   *RookMoveGen
	QueenMoveGen  *QueenMoveGen
	KingMoveGen   *KingMoveGen
}

func NewMoveGen() *MoveGen {
	pawnMoveGen := NewPawnMoveGen()
	knightMoveGen := NewKnightMoveGen()
	bishopMoveGen := NewBishopMoveGen()
	rookMoveGen := NewRookMoveGen()
	queenMoveGen := NewQueenMoveGen(rookMoveGen, bishopMoveGen)
	kingMoveGen := NewKingMoveGen()

	return &MoveGen{
		PawnMoveGen:   pawnMoveGen,
		KnightMoveGen: knightMoveGen,
		BishopMoveGen: bishopMoveGen,
		RookMoveGen:   rookMoveGen,
		QueenMoveGen:  queenMoveGen,
		KingMoveGen:   kingMoveGen,
	}
}

func (mg *MoveGen) IsSquareAttacked(pos *position.Position, square uint8, attackerColor uint8) bool {
	if attackerColor == constants.ColorWhite {
		if mg.PawnMoveGen.GetAttacks(square, constants.ColorBlack)&pos.PieceBitboards[constants.WhitePawn] != 0 {
			return true
		}

		if mg.KnightMoveGen.GetAttacks(square)&pos.PieceBitboards[constants.WhiteKnight] != 0 {
			return true
		}

		if mg.KingMoveGen.GetAttacks(square)&pos.PieceBitboards[constants.WhiteKing] != 0 {
			return true
		}

		if mg.BishopMoveGen.GetAttacks(square, pos.ColorBitboards[constants.ColorBoth])&pos.PieceBitboards[constants.WhiteBishop] != 0 {
			return true
		}

		if mg.RookMoveGen.GetAttacks(square, pos.ColorBitboards[constants.ColorBoth])&pos.PieceBitboards[constants.WhiteRook] != 0 {
			return true
		}

		if mg.QueenMoveGen.GetAttacks(square, pos.ColorBitboards[constants.ColorBoth])&pos.PieceBitboards[constants.WhiteQueen] != 0 {
			return true
		}
	} else if attackerColor == constants.ColorBlack {
		if mg.PawnMoveGen.GetAttacks(square, constants.ColorWhite)&pos.PieceBitboards[constants.BlackPawn] != 0 {
			return true
		}

		if mg.KnightMoveGen.GetAttacks(square)&pos.PieceBitboards[constants.BlackKnight] != 0 {
			return true
		}

		if mg.KingMoveGen.GetAttacks(square)&pos.PieceBitboards[constants.BlackKing] != 0 {
			return true
		}

		if mg.BishopMoveGen.GetAttacks(square, pos.ColorBitboards[constants.ColorBoth])&pos.PieceBitboards[constants.BlackBishop] != 0 {
			return true
		}

		if mg.RookMoveGen.GetAttacks(square, pos.ColorBitboards[constants.ColorBoth])&pos.PieceBitboards[constants.BlackRook] != 0 {
			return true
		}

		if mg.QueenMoveGen.GetAttacks(square, pos.ColorBitboards[constants.ColorBoth])&pos.PieceBitboards[constants.BlackQueen] != 0 {
			return true
		}
	}

	return false
}

func (mg *MoveGen) GeneratePseudoLegalMoves(pos *position.Position) []move.Move {
	moves := []move.Move{}

	moves = append(moves, mg.PawnMoveGen.GeneratePseudoLegalMoves(pos)...)
	moves = append(moves, mg.KnightMoveGen.GeneratePseudoLegalMoves(pos)...)
	moves = append(moves, mg.BishopMoveGen.GeneratePseudoLegalMoves(pos)...)
	moves = append(moves, mg.RookMoveGen.GeneratePseudoLegalMoves(pos)...)
	moves = append(moves, mg.QueenMoveGen.GeneratePseudoLegalMoves(pos)...)
	moves = append(moves, mg.KingMoveGen.GeneratePseudoLegalMoves(pos, mg)...)

	return moves
}

type MoveWithNewPosition struct {
	Move     move.Move
	Position position.Position
}

func (mg *MoveGen) GenerateLegalMovesExpensive(pos *position.Position) []MoveWithNewPosition {
	legalMoves := []MoveWithNewPosition{}
	pseudoLegalMoves := mg.GeneratePseudoLegalMoves(pos)

	for _, pseudoLegalMove := range pseudoLegalMoves {
		newPos := pos.MakeMove(mg, &pseudoLegalMove, false)
		if newPos != nil {
			legalMoves = append(legalMoves, MoveWithNewPosition{
				Move:     pseudoLegalMove,
				Position: *newPos,
			})
		}
	}

	return legalMoves
}
