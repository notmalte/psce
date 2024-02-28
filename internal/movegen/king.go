package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
)

type KingMoveGen struct{}

func (kmg *KingMoveGen) GenerateAttackTable() [64]uint64 {
	table := [64]uint64{}

	for square := range uint8(64) {
		table[square] = kmg.MaskAttacks(square)
	}

	return table
}

func (kmg *KingMoveGen) MaskAttacks(square uint8) uint64 {
	attacks := uint64(0)
	bb := uint64(0)
	bitboard.SetBit(&bb, square)

	attacks |= (bb >> 9) & constants.NotFileH
	attacks |= bb >> 8
	attacks |= (bb >> 7) & constants.NotFileA
	attacks |= (bb >> 1) & constants.NotFileH
	attacks |= (bb << 1) & constants.NotFileA
	attacks |= (bb << 7) & constants.NotFileH
	attacks |= bb << 8
	attacks |= (bb << 9) & constants.NotFileA

	return attacks
}
