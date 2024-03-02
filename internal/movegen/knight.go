package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
)

type KnightMoveGen struct {
	attackTable [64]uint64
}

func (kmg *KnightMoveGen) generateAttackTable() [64]uint64 {
	table := [64]uint64{}

	for square := range uint8(64) {
		table[square] = kmg.maskAttacks(square)
	}

	return table
}

func (kmg *KnightMoveGen) maskAttacks(square uint8) uint64 {
	attacks := uint64(0)
	bb := uint64(0)
	bitboard.SetBit(&bb, square)

	attacks |= (bb >> 17) & constants.NotFileH
	attacks |= (bb >> 15) & constants.NotFileA
	attacks |= (bb >> 10) & constants.NotFileGH
	attacks |= (bb >> 6) & constants.NotFileAB
	attacks |= (bb << 6) & constants.NotFileGH
	attacks |= (bb << 10) & constants.NotFileAB
	attacks |= (bb << 15) & constants.NotFileH
	attacks |= (bb << 17) & constants.NotFileA

	return attacks
}

func (kmg *KnightMoveGen) GetAttacks(square uint8) uint64 {
	return kmg.attackTable[square]
}

func NewKnightMoveGen() *KnightMoveGen {
	kmg := &KnightMoveGen{}
	kmg.attackTable = kmg.generateAttackTable()

	return kmg
}

func (kmg *KnightMoveGen) GeneratePseudoLegalMoves(pos *position.Position) []Move {
	isWhite := pos.ColorToMove == constants.ColorWhite

	var otherColor uint8
	var piece uint8
	if isWhite {
		otherColor = constants.ColorBlack
		piece = constants.WhiteKnight
	} else {
		otherColor = constants.ColorWhite
		piece = constants.BlackKnight
	}

	moves := []Move{}
	bb := pos.PieceBitboards[piece]

	for bb != 0 {
		fromSquare := uint8(bits.TrailingZeros64(bb))

		attacks := kmg.GetAttacks(fromSquare) & ^pos.ColorBitboards[pos.ColorToMove]

		for attacks != 0 {
			toSquare := uint8(bits.TrailingZeros64(attacks))

			flags := FlagNone
			if bitboard.GetBit(pos.ColorBitboards[otherColor], toSquare) {
				flags = FlagCapture
			}

			moves = append(moves, Move{
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
