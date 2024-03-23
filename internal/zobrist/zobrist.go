package zobrist

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
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

	hash ^= k.Castling[pos.CastlingRights]

	if pos.ColorToMove == constants.ColorWhite {
		hash ^= k.Side
	}

	return hash
}

func (k *Keys) IncrementalHash(hash uint64, oldPos *position.Position, newPos *position.Position, m *move.Move) uint64 {
	isWhite := oldPos.ColorToMove == constants.ColorWhite

	hash ^= k.Castling[oldPos.CastlingRights]
	hash ^= k.Castling[newPos.CastlingRights]

	if oldPos.EnPassantSquare != constants.NoSquare {
		hash ^= k.EnPassant[oldPos.EnPassantSquare]
	}
	if newPos.EnPassantSquare != constants.NoSquare {
		hash ^= k.EnPassant[newPos.EnPassantSquare]
	}

	hash ^= k.PieceSquare[m.Piece][m.FromSquare]

	if m.HasFlag(constants.MoveFlagPromotion) {
		hash ^= k.PieceSquare[m.PromotionPiece][m.ToSquare]
	} else {
		hash ^= k.PieceSquare[m.Piece][m.ToSquare]
	}

	if m.HasFlag(constants.MoveFlagCapture) {
		if m.HasFlag(constants.MoveFlagEnPassant) {
			var capturedPawnSquare uint8
			var capturedPawnPiece uint8
			if isWhite {
				capturedPawnSquare = m.ToSquare + 8
				capturedPawnPiece = constants.BlackPawn
			} else {
				capturedPawnSquare = m.ToSquare - 8
				capturedPawnPiece = constants.WhitePawn
			}

			hash ^= k.PieceSquare[capturedPawnPiece][capturedPawnSquare]
		} else {
			var opponentPieces [6]uint8
			if isWhite {
				opponentPieces = [6]uint8{constants.BlackPawn, constants.BlackKnight, constants.BlackBishop, constants.BlackRook, constants.BlackQueen, constants.BlackKing}
			} else {
				opponentPieces = [6]uint8{constants.WhitePawn, constants.WhiteKnight, constants.WhiteBishop, constants.WhiteRook, constants.WhiteQueen, constants.WhiteKing}
			}

			for _, piece := range opponentPieces {
				if bitboard.GetBit(oldPos.PieceBitboards[piece], m.ToSquare) {
					hash ^= k.PieceSquare[piece][m.ToSquare]
					break
				}
			}
		}
	}

	if m.HasFlag(constants.MoveFlagCastle) {
		var rookFromSquare uint8
		var rookToSquare uint8
		switch m.ToSquare {
		case constants.C8:
			rookFromSquare, rookToSquare = constants.A8, constants.D8
		case constants.G8:
			rookFromSquare, rookToSquare = constants.H8, constants.F8
		case constants.C1:
			rookFromSquare, rookToSquare = constants.A1, constants.D1
		case constants.G1:
			rookFromSquare, rookToSquare = constants.H1, constants.F1
		default:
			panic("Invalid castling move")
		}

		var rookPiece uint8
		if isWhite {
			rookPiece = constants.WhiteRook
		} else {
			rookPiece = constants.BlackRook
		}

		hash ^= k.PieceSquare[rookPiece][rookFromSquare]
		hash ^= k.PieceSquare[rookPiece][rookToSquare]
	}

	hash ^= k.Side

	return hash
}
