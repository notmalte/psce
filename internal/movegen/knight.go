package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
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
