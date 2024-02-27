package bitboard

import (
	"github.com/notmalte/psce/internal/constants"
	"testing"
)

func TestGetRowCol(t *testing.T) {
	bb := Bitboard(1 << constants.B3)

	got := bb.GetRowCol(5, 1) // B3
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = bb.GetRowCol(5, 2) // C3
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestSetRowCol(t *testing.T) {
	bb := Bitboard(0)

	got := bb.GetRowCol(1, 6) // G7
	if got {
		t.Errorf("Expected false, got true")
	}

	bb.SetRowCol(1, 6) // G7
	got = bb.GetRowCol(1, 6)
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = bb.GetRowCol(2, 6) // G6
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestClearRowCol(t *testing.T) {
	bb := Bitboard(0)

	bb.SetRowCol(1, 6) // G7
	got := bb.GetRowCol(1, 6)
	if !got {
		t.Errorf("Expected true, got false")
	}

	bb.ClearRowCol(1, 6)
	got = bb.GetRowCol(1, 6)
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestGetIndex(t *testing.T) {
	bb := Bitboard(1 << constants.B3)

	got := bb.GetBit(constants.B3)
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = bb.GetBit(constants.C3)
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestSetIndex(t *testing.T) {
	bb := Bitboard(0)

	got := bb.GetBit(constants.G7)
	if got {
		t.Errorf("Expected false, got true")
	}

	bb.SetBit(constants.G7)
	got = bb.GetBit(constants.G7)
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = bb.GetBit(constants.G6)
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestClearIndex(t *testing.T) {
	bb := Bitboard(0)

	bb.SetBit(constants.G7)
	got := bb.GetBit(constants.G7)
	if !got {
		t.Errorf("Expected true, got false")
	}

	bb.ClearBit(constants.G7)
	got = bb.GetBit(constants.G7)
	if got {
		t.Errorf("Expected false, got true")
	}
}
