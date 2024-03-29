package bitboard

import (
	"github.com/notmalte/psce/internal/constants"
	"testing"
)

func TestIndexToRowCol(t *testing.T) {
	row, col := IndexToRowCol(constants.G5)
	if row != 3 || col != 6 {
		t.Errorf("Expected (3, 6), got (%d, %d)", row, col)
	}
}

func TestRowColToIndex(t *testing.T) {
	index := RowColToIndex(3, 6)
	if index != constants.G5 {
		t.Errorf("Expected %d, got %d", constants.G5, index)
	}
}

func TestGetBit(t *testing.T) {
	bb := uint64(1 << constants.B3)

	got := GetBit(bb, constants.B3)
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = GetBit(bb, constants.C3)
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestSetBit(t *testing.T) {
	bb := uint64(0)

	got := GetBit(bb, constants.G7)
	if got {
		t.Errorf("Expected false, got true")
	}

	SetBit(&bb, constants.G7)
	got = GetBit(bb, constants.G7)
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = GetBit(bb, constants.G6)
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestClearBit(t *testing.T) {
	bb := uint64(0)

	SetBit(&bb, constants.G7)
	got := GetBit(bb, constants.G7)
	if !got {
		t.Errorf("Expected true, got false")
	}

	ClearBit(&bb, constants.G7)
	got = GetBit(bb, constants.G7)
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestGetRowCol(t *testing.T) {
	bb := uint64(1 << constants.B3)

	got := GetRowCol(bb, 5, 1) // B3
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = GetRowCol(bb, 5, 2) // C3
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestSetRowCol(t *testing.T) {
	bb := uint64(0)

	got := GetRowCol(bb, 1, 6) // G7
	if got {
		t.Errorf("Expected false, got true")
	}

	SetRowCol(&bb, 1, 6) // G7
	got = GetRowCol(bb, 1, 6)
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = GetRowCol(bb, 2, 6) // G6
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestClearRowCol(t *testing.T) {
	bb := uint64(0)

	SetRowCol(&bb, 1, 6) // G7
	got := GetRowCol(bb, 1, 6)
	if !got {
		t.Errorf("Expected true, got false")
	}

	ClearRowCol(&bb, 1, 6)
	got = GetRowCol(bb, 1, 6)
	if got {
		t.Errorf("Expected false, got true")
	}
}
