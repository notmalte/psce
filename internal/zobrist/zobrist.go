package zobrist

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
	"math/rand/v2"
)

type Keys struct {
	PieceSquare [12][64]uint64
	EnPassant   [64]uint64
	Castling    [16]uint64
	WhiteToMove uint64
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

	r.WhiteToMove = rand.Uint64()

	return r
}

func (k *Keys) GenerateHash(pos *position.Position) uint64 {
	hash := uint64(0)

	for piece := range constants.PiecesCount {
		bb := pos.PieceBitboards[piece]
		for bb != 0 {
			square := uint8(bits.TrailingZeros64(bb))
			hash ^= k.PieceSquare[piece][square]
			bitboard.ClearBit(&bb, square)
		}
	}

	if pos.EnPassantSquare != constants.NoSquare {
		hash ^= k.EnPassant[pos.EnPassantSquare]
	}

	if pos.CastlingRights != 0 {
		hash ^= k.Castling[pos.CastlingRights]
	}

	if pos.ColorToMove == constants.ColorWhite {
		hash ^= k.WhiteToMove
	}

	return hash
}
