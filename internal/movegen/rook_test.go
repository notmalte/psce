package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"testing"
)

func TestRookMaskAttackCandidates(t *testing.T) {
	rmg := RookMoveGen{}

	got := rmg.maskAttackCandidates(constants.F7)

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.B7)
	bitboard.SetBit(&expected, constants.C7)
	bitboard.SetBit(&expected, constants.D7)
	bitboard.SetBit(&expected, constants.E7)
	bitboard.SetBit(&expected, constants.G7)
	bitboard.SetBit(&expected, constants.F6)
	bitboard.SetBit(&expected, constants.F5)
	bitboard.SetBit(&expected, constants.F4)
	bitboard.SetBit(&expected, constants.F3)
	bitboard.SetBit(&expected, constants.F2)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}
}

func TestRookGenerateAttackCandidateTable(t *testing.T) {
	rmg := RookMoveGen{}

	table := rmg.generateAttackCandidateTable()

	got := table[constants.G3]

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.G7)
	bitboard.SetBit(&expected, constants.G6)
	bitboard.SetBit(&expected, constants.G5)
	bitboard.SetBit(&expected, constants.G4)
	bitboard.SetBit(&expected, constants.B3)
	bitboard.SetBit(&expected, constants.C3)
	bitboard.SetBit(&expected, constants.D3)
	bitboard.SetBit(&expected, constants.E3)
	bitboard.SetBit(&expected, constants.F3)
	bitboard.SetBit(&expected, constants.G2)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}
}

func TestRookMaskAttacks(t *testing.T) {
	rmg := RookMoveGen{}

	occupancy := uint64(0)
	bitboard.SetBit(&occupancy, constants.D6)
	bitboard.SetBit(&occupancy, constants.A3)
	bitboard.SetBit(&occupancy, constants.F3)

	got := rmg.maskAttacks(constants.D3, occupancy)

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.D6)
	bitboard.SetBit(&expected, constants.D5)
	bitboard.SetBit(&expected, constants.D4)
	bitboard.SetBit(&expected, constants.A3)
	bitboard.SetBit(&expected, constants.B3)
	bitboard.SetBit(&expected, constants.C3)
	bitboard.SetBit(&expected, constants.E3)
	bitboard.SetBit(&expected, constants.F3)
	bitboard.SetBit(&expected, constants.D2)
	bitboard.SetBit(&expected, constants.D1)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}
}
