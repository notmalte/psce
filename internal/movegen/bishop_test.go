package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"testing"
)

func TestBishopMaskAttackCandidates(t *testing.T) {
	bmg := BishopMoveGen{}

	got := bmg.MaskAttackCandidates(constants.F6)

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.E7)
	bitboard.SetBit(&expected, constants.G7)
	bitboard.SetBit(&expected, constants.E5)
	bitboard.SetBit(&expected, constants.G5)
	bitboard.SetBit(&expected, constants.D4)
	bitboard.SetBit(&expected, constants.C3)
	bitboard.SetBit(&expected, constants.B2)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}
}

func TestBishopGenerateAttackCandidateTable(t *testing.T) {
	bmg := BishopMoveGen{}

	table := bmg.GenerateAttackCandidateTable()

	got := table[constants.A6]

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.B7)
	bitboard.SetBit(&expected, constants.B5)
	bitboard.SetBit(&expected, constants.C4)
	bitboard.SetBit(&expected, constants.D3)
	bitboard.SetBit(&expected, constants.E2)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}

}
