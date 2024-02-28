package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"testing"
)

func TestKingMaskAttacks(t *testing.T) {
	kmg := KingMoveGen{}

	got := kmg.MaskAttacks(constants.E1)

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.D1)
	bitboard.SetBit(&expected, constants.D2)
	bitboard.SetBit(&expected, constants.E2)
	bitboard.SetBit(&expected, constants.F1)
	bitboard.SetBit(&expected, constants.F2)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}

	got = kmg.MaskAttacks(constants.E4)

	expected = uint64(0)
	bitboard.SetBit(&expected, constants.D3)
	bitboard.SetBit(&expected, constants.D4)
	bitboard.SetBit(&expected, constants.D5)
	bitboard.SetBit(&expected, constants.E3)
	bitboard.SetBit(&expected, constants.E5)
	bitboard.SetBit(&expected, constants.F3)
	bitboard.SetBit(&expected, constants.F4)
	bitboard.SetBit(&expected, constants.F5)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}
}

func TestKingGenerateAttackTable(t *testing.T) {
	kmg := KingMoveGen{}

	table := kmg.GenerateAttackTable()

	got := table[constants.C7]

	expected := uint64(0)
	bitboard.SetBit(&expected, constants.B6)
	bitboard.SetBit(&expected, constants.B7)
	bitboard.SetBit(&expected, constants.B8)
	bitboard.SetBit(&expected, constants.C6)
	bitboard.SetBit(&expected, constants.C8)
	bitboard.SetBit(&expected, constants.D6)
	bitboard.SetBit(&expected, constants.D7)
	bitboard.SetBit(&expected, constants.D8)

	if got != expected {
		t.Errorf("Expected %d, got %d", expected, got)
	}

}
