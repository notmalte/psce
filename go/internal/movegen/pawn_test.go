package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"testing"
)

func TestPawnMaskAttacks(t *testing.T) {
	pmg := PawnMoveGen{}

	got := pmg.maskAttacks(constants.ColorWhite, constants.A2)

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.B3)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}

	got = pmg.maskAttacks(constants.ColorBlack, constants.A7)

	expected = uint64(0)
	bitboard.SetBit(&expected, constants.B6)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}

	got = pmg.maskAttacks(constants.ColorWhite, constants.B4)

	expected = uint64(0)
	bitboard.SetBit(&expected, constants.A5)
	bitboard.SetBit(&expected, constants.C5)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}
}

func TestPawnGenerateAttackTable(t *testing.T) {
	pmg := PawnMoveGen{}

	table := pmg.generateAttackTable()

	got := table[constants.ColorWhite][constants.E3]

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.D4)
	bitboard.SetBit(&expected, constants.F4)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}

}
