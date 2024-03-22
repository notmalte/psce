package zobrist

import (
	"math/rand/v2"
)

type Keys struct {
	PieceSquare [12][64]uint64
	EnPassant   [64]uint64
	Castling    [16]uint64
	Side        uint64
}

func NewKeys() *Keys {
	r := &Keys{}

	for i := 0; i < 12; i++ {
		for j := 0; j < 64; j++ {
			r.PieceSquare[i][j] = rand.Uint64()
		}
	}

	for i := 0; i < 64; i++ {
		r.EnPassant[i] = rand.Uint64()
	}

	for i := 0; i < 16; i++ {
		r.Castling[i] = rand.Uint64()
	}

	r.Side = rand.Uint64()

	return r
}
