package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"testing"
)

func TestKnightMaskAttacks(t *testing.T) {
	kmg := KnightMoveGen{}

	got := kmg.maskAttacks(constants.A2)

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.B4)
	bitboard.SetBit(&expected, constants.C3)
	bitboard.SetBit(&expected, constants.C1)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}

	got = kmg.maskAttacks(constants.D4)

	expected = uint64(0)
	bitboard.SetBit(&expected, constants.B3)
	bitboard.SetBit(&expected, constants.B5)
	bitboard.SetBit(&expected, constants.C2)
	bitboard.SetBit(&expected, constants.C6)
	bitboard.SetBit(&expected, constants.E2)
	bitboard.SetBit(&expected, constants.E6)
	bitboard.SetBit(&expected, constants.F3)
	bitboard.SetBit(&expected, constants.F5)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}
}

func TestKnightGenerateAttackTable(t *testing.T) {
	kmg := KnightMoveGen{}

	table := kmg.generateAttackTable()

	got := table[constants.E3]

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.C2)
	bitboard.SetBit(&expected, constants.C4)
	bitboard.SetBit(&expected, constants.D1)
	bitboard.SetBit(&expected, constants.D5)
	bitboard.SetBit(&expected, constants.F1)
	bitboard.SetBit(&expected, constants.F5)
	bitboard.SetBit(&expected, constants.G2)
	bitboard.SetBit(&expected, constants.G4)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}

}
