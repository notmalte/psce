package bitboard

import (
	"testing"
)

func TestGetRowCol(t *testing.T) {
	bb := Bitboard(1 << B3)

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
	bb := Bitboard(1 << B3)

	got := bb.GetIndex(B3)
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = bb.GetIndex(C3)
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestSetIndex(t *testing.T) {
	bb := Bitboard(0)

	got := bb.GetIndex(G7)
	if got {
		t.Errorf("Expected false, got true")
	}

	bb.SetIndex(G7)
	got = bb.GetIndex(G7)
	if !got {
		t.Errorf("Expected true, got false")
	}

	got = bb.GetIndex(G6)
	if got {
		t.Errorf("Expected false, got true")
	}
}

func TestClearIndex(t *testing.T) {
	bb := Bitboard(0)

	bb.SetIndex(G7)
	got := bb.GetIndex(G7)
	if !got {
		t.Errorf("Expected true, got false")
	}

	bb.ClearIndex(G7)
	got = bb.GetIndex(G7)
	if got {
		t.Errorf("Expected false, got true")
	}
}
