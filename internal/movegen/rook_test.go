package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"testing"
)

func TestRookMaskAttackCandidates(t *testing.T) {
	rmg := RookMoveGen{}

	got := rmg.MaskAttackCandidates(constants.F7)

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

	table := rmg.GenerateAttackCandidateTable()

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
