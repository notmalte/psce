package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
)

type PawnMoveGen struct {
	attackTable [2][64]uint64
}

func (pmg *PawnMoveGen) generateAttackTable() [2][64]uint64 {
	table := [2][64]uint64{}

	for square := range uint8(64) {
		table[constants.ColorWhite][square] = pmg.maskAttacks(constants.ColorWhite, square)
		table[constants.ColorBlack][square] = pmg.maskAttacks(constants.ColorBlack, square)
	}

	return table
}

func (pmg *PawnMoveGen) maskAttacks(color uint8, square uint8) uint64 {
	attacks := uint64(0)
	bb := uint64(0)
	bitboard.SetBit(&bb, square)

	if color == constants.ColorWhite {
		attacks |= (bb >> 7) & constants.NotFileA
		attacks |= (bb >> 9) & constants.NotFileH
	} else {
		attacks |= (bb << 7) & constants.NotFileH
		attacks |= (bb << 9) & constants.NotFileA
	}

	return attacks
}

func (pmg *PawnMoveGen) GetAttacks(square uint8, color uint8) uint64 {
	return pmg.attackTable[color][square]
}

func NewPawnMoveGen() *PawnMoveGen {
	pmg := &PawnMoveGen{}
	pmg.attackTable = pmg.generateAttackTable()

	return pmg
}
