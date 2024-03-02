package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
)

type PawnMoveGen struct {
	attackTable [2][64]uint64
}

func (pmg *PawnMoveGen) generateAttackTable() [2][64]uint64 {
	table := [2][64]uint64{}

	for square := range uint8(64) {
		table[constants.ColorWhite][square] = pmg.maskAttacks(constants.ColorWhite, square)
		table[constants.ColorBlack][square] = pmg.maskAttacks(constants.ColorBlack, square)
	}

	return table
}

func (pmg *PawnMoveGen) maskAttacks(color uint8, square uint8) uint64 {
	attacks := uint64(0)
	bb := uint64(0)
	bitboard.SetBit(&bb, square)

	if color == constants.ColorWhite {
		attacks |= (bb >> 7) & constants.NotFileA
		attacks |= (bb >> 9) & constants.NotFileH
	} else {
		attacks |= (bb << 7) & constants.NotFileH
		attacks |= (bb << 9) & constants.NotFileA
	}

	return attacks
}

func (pmg *PawnMoveGen) GetAttacks(square uint8, color uint8) uint64 {
	return pmg.attackTable[color][square]
}

func NewPawnMoveGen() *PawnMoveGen {
	pmg := &PawnMoveGen{}
	pmg.attackTable = pmg.generateAttackTable()

	return pmg
}

func (pmg *PawnMoveGen) GeneratePseudoLegalMoves(pos *position.Position) []Move {
	isWhite := pos.ColorToMove == constants.ColorWhite

	var otherColor uint8
	var piece uint8
	var promotionRowStart uint8
	var promotionRowEnd uint8
	var homeRowStart uint8
	var homeRowEnd uint8
	if isWhite {
		otherColor = constants.ColorBlack
		piece = constants.WhitePawn
		promotionRowStart = constants.A8
		promotionRowEnd = constants.H8
		homeRowStart = constants.A2
		homeRowEnd = constants.H2
	} else {
		otherColor = constants.ColorWhite
		piece = constants.BlackPawn
		promotionRowStart = constants.A1
		promotionRowEnd = constants.H1
		homeRowStart = constants.A7
		homeRowEnd = constants.H7
	}

	moves := []Move{}
	bb := pos.PieceBitboards[piece]

	/*
		TODO: this can be done by shifting and masking,
			<< +-8 for single push, << +-16 for double push,
			& with constants.NotFileA and constants.NotFileH for file checks
		 	<< +-7 and << +-9 for captures
	*/

	for bb != 0 {
		fromSquare := uint8(bits.TrailingZeros64(bb))

		if fromSquare >= promotionRowStart && fromSquare <= promotionRowEnd {
			continue
		}

		var toSquare uint8
		if isWhite {
			toSquare = fromSquare - 8
		} else {
			toSquare = fromSquare + 8
		}

		if !bitboard.GetBit(pos.ColorBitboards[constants.ColorBoth], toSquare) {
			if toSquare >= promotionRowStart && toSquare <= promotionRowEnd {
				var promotionPieces [4]uint8
				if isWhite {
					promotionPieces = [4]uint8{constants.WhiteQueen, constants.WhiteRook, constants.WhiteBishop, constants.WhiteKnight}
				} else {
					promotionPieces = [4]uint8{constants.BlackQueen, constants.BlackRook, constants.BlackBishop, constants.BlackKnight}
				}

				for _, promotionPiece := range promotionPieces {
					moves = append(moves, Move{
						FromSquare:     fromSquare,
						ToSquare:       toSquare,
						Piece:          piece,
						Flags:          FlagPromotion,
						PromotionPiece: promotionPiece,
					})
				}
			} else {
				moves = append(moves, Move{
					FromSquare: fromSquare,
					ToSquare:   toSquare,
					Piece:      piece,
				})

				if fromSquare >= homeRowStart && fromSquare <= homeRowEnd {
					var doublePushSquare uint8
					if isWhite {
						doublePushSquare = fromSquare - 16
					} else {
						doublePushSquare = fromSquare + 16
					}

					if !bitboard.GetBit(pos.ColorBitboards[constants.ColorBoth], doublePushSquare) {
						moves = append(moves, Move{
							FromSquare: fromSquare,
							ToSquare:   doublePushSquare,
							Piece:      piece,
							Flags:      FlagDoublePawnPush,
						})
					}
				}
			}
		}

		attacks := pmg.GetAttacks(fromSquare, pos.ColorToMove)
		captures := attacks & pos.ColorBitboards[otherColor]

		for captures != 0 {
			toSquare = uint8(bits.TrailingZeros64(captures))

			if toSquare >= promotionRowStart && toSquare <= promotionRowEnd {
				var promotionPieces [4]uint8
				if isWhite {
					promotionPieces = [4]uint8{constants.WhiteQueen, constants.WhiteRook, constants.WhiteBishop, constants.WhiteKnight}
				} else {
					promotionPieces = [4]uint8{constants.BlackQueen, constants.BlackRook, constants.BlackBishop, constants.BlackKnight}
				}

				for _, promotionPiece := range promotionPieces {
					moves = append(moves, Move{
						FromSquare:     fromSquare,
						ToSquare:       toSquare,
						Piece:          piece,
						Flags:          FlagPromotion | FlagCapture,
						PromotionPiece: promotionPiece,
					})
				}
			} else {
				moves = append(moves, Move{
					FromSquare: fromSquare,
					ToSquare:   toSquare,
					Piece:      piece,
					Flags:      FlagCapture,
				})
			}

			bitboard.ClearBit(&captures, toSquare)
		}

		if pos.EnPassantSquare != constants.NoSquare {
			if bitboard.GetBit(attacks, pos.EnPassantSquare) {
				moves = append(moves, Move{
					FromSquare: fromSquare,
					ToSquare:   pos.EnPassantSquare,
					Piece:      piece,
					Flags:      FlagEnPassant | FlagCapture,
				})
			}
		}

		bitboard.ClearBit(&bb, fromSquare)
	}

	return moves
}
