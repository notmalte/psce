package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
)

type QueenMoveGen struct {
	rookMoveGen   *RookMoveGen
	bishopMoveGen *BishopMoveGen
}

func (qmg *QueenMoveGen) GetAttacks(square uint8, occupancy uint64) uint64 {
	return qmg.rookMoveGen.GetAttacks(square, occupancy) | qmg.bishopMoveGen.GetAttacks(square, occupancy)
}

func NewQueenMoveGen(rmg *RookMoveGen, bmg *BishopMoveGen) *QueenMoveGen {
	return &QueenMoveGen{
		rookMoveGen:   rmg,
		bishopMoveGen: bmg,
	}
}

func (qmg *QueenMoveGen) GeneratePseudoLegalMoves(pos *position.Position) []move.Move {
	isWhite := pos.ColorToMove == constants.ColorWhite

	var otherColor uint8
	var piece uint8
	if isWhite {
		otherColor = constants.ColorBlack
		piece = constants.WhiteQueen
	} else {
		otherColor = constants.ColorWhite
		piece = constants.BlackQueen
	}

	moves := []move.Move{}
	bb := pos.PieceBitboards[piece]

	for bb != 0 {
		fromSquare := uint8(bits.TrailingZeros64(bb))

		attacks := qmg.GetAttacks(fromSquare, pos.ColorBitboards[constants.ColorBoth]) & ^pos.ColorBitboards[pos.ColorToMove]

		for attacks != 0 {
			toSquare := uint8(bits.TrailingZeros64(attacks))

			flags := constants.MoveFlagNone
			if bitboard.GetBit(pos.ColorBitboards[otherColor], toSquare) {
				flags = constants.MoveFlagCapture
			}

			moves = append(moves, move.Move{
				FromSquare: fromSquare,
				ToSquare:   toSquare,
				Piece:      piece,
				Flags:      flags,
			})

			bitboard.ClearBit(&attacks, toSquare)
		}

		bitboard.ClearBit(&bb, fromSquare)
	}

	return moves
}
