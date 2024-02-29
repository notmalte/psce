package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"math/bits"
)

func MaskOccupancy(attackMask uint64, index uint64) uint64 {
	occupancy := uint64(0)
	bitsInMask := bits.OnesCount64(attackMask)

	for count := 0; count < bitsInMask; count++ {
		square := uint8(bits.TrailingZeros64(attackMask))
		bitboard.ClearBit(&attackMask, square)

		if bitboard.GetBit(index, uint8(count)) {
			bitboard.SetBit(&occupancy, square)
		}
	}

	return occupancy
}
