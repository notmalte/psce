package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
)

type PawnMoveGen struct{}

func (pmg *PawnMoveGen) GenerateAttackTable() [2][64]uint64 {
	table := [2][64]uint64{}

	for square := range uint8(64) {
		table[constants.ColorWhite][square] = pmg.MaskAttacks(constants.ColorWhite, square)
		table[constants.ColorBlack][square] = pmg.MaskAttacks(constants.ColorBlack, square)
	}

	return table
}

func (pmg *PawnMoveGen) MaskAttacks(color uint8, square uint8) uint64 {
	attacks := uint64(0)
	bb := bitboard.Bitboard(0)
	bb.SetBit(square)

	if color == constants.ColorWhite {
		attacks |= (uint64(bb) >> 7) & constants.NotFileA
		attacks |= (uint64(bb) >> 9) & constants.NotFileH
	} else {
		attacks |= (uint64(bb) << 7) & constants.NotFileH
		attacks |= (uint64(bb) << 9) & constants.NotFileA
	}

	return attacks
}
