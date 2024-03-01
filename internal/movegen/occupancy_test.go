package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"math/bits"
	"testing"
)

func TestMaskOccupancy(t *testing.T) {
	rmg := RookMoveGen{}
	candidateMask := rmg.maskAttackCandidates(constants.A1)
	bitsInMask := bits.OnesCount64(candidateMask)
	maxIndex := uint64((1 << bitsInMask) - 1)

	if maxIndex != 4095 {
		t.Errorf("Expected 4095, got %d", maxIndex)
	}

	occupancy := maskOccupancy(candidateMask, 0)
	expected := uint64(0)
	if occupancy != expected {
		t.Errorf("Expected %d, got %d", expected, occupancy)
	}

	occupancy = maskOccupancy(candidateMask, 1)
	expected = uint64(0)
	bitboard.SetBit(&expected, constants.A7)
	if occupancy != expected {
		t.Errorf("Expected %d, got %d", expected, occupancy)
	}

	occupancy = maskOccupancy(candidateMask, maxIndex)
	expected = uint64(0)
	bitboard.SetBit(&expected, constants.A7)
	bitboard.SetBit(&expected, constants.A6)
	bitboard.SetBit(&expected, constants.A5)
	bitboard.SetBit(&expected, constants.A4)
	bitboard.SetBit(&expected, constants.A3)
	bitboard.SetBit(&expected, constants.A2)
	bitboard.SetBit(&expected, constants.B1)
	bitboard.SetBit(&expected, constants.C1)
	bitboard.SetBit(&expected, constants.D1)
	bitboard.SetBit(&expected, constants.E1)
	bitboard.SetBit(&expected, constants.F1)
	bitboard.SetBit(&expected, constants.G1)
	if occupancy != expected {
		t.Errorf("Expected %d, got %d", expected, occupancy)
	}
}
