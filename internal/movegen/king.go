package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
)

type KingMoveGen struct {
	attackTable [64]uint64
}

func (kmg *KingMoveGen) generateAttackTable() [64]uint64 {
	table := [64]uint64{}

	for square := range uint8(64) {
		table[square] = kmg.maskAttacks(square)
	}

	return table
}

func (kmg *KingMoveGen) maskAttacks(square uint8) uint64 {
	attacks := uint64(0)
	bb := uint64(0)
	bitboard.SetBit(&bb, square)

	attacks |= (bb >> 9) & constants.NotFileH
	attacks |= bb >> 8
	attacks |= (bb >> 7) & constants.NotFileA
	attacks |= (bb >> 1) & constants.NotFileH
	attacks |= (bb << 1) & constants.NotFileA
	attacks |= (bb << 7) & constants.NotFileH
	attacks |= bb << 8
	attacks |= (bb << 9) & constants.NotFileA

	return attacks
}

func (kmg *KingMoveGen) GetAttacks(square uint8) uint64 {
	return kmg.attackTable[square]
}

func NewKingMoveGen() *KingMoveGen {
	kmg := &KingMoveGen{}
	kmg.attackTable = kmg.generateAttackTable()

	return kmg
}

func (kmg *KingMoveGen) GeneratePseudoLegalMoves(pos *position.Position, mg *MoveGen) []Move {
	isWhite := pos.ColorToMove == constants.ColorWhite

	var otherColor uint8
	var piece uint8
	var kingsideCastleFlag uint8
	var queensideCastleFlag uint8
	if isWhite {
		otherColor = constants.ColorBlack
		piece = constants.WhiteKing
		kingsideCastleFlag = constants.CastlingWhiteKingside
		queensideCastleFlag = constants.CastlingWhiteQueenside
	} else {
		otherColor = constants.ColorWhite
		piece = constants.BlackKing
		kingsideCastleFlag = constants.CastlingBlackKingside
		queensideCastleFlag = constants.CastlingBlackQueenside
	}

	moves := []Move{}
	bb := pos.PieceBitboards[piece]

	for bb != 0 {
		fromSquare := uint8(bits.TrailingZeros64(bb))

		attacks := kmg.GetAttacks(fromSquare) & ^pos.ColorBitboards[pos.ColorToMove]

		for attacks != 0 {
			toSquare := uint8(bits.TrailingZeros64(attacks))

			flags := FlagNone
			if bitboard.GetBit(pos.ColorBitboards[otherColor], toSquare) {
				flags = FlagCapture
			}

			moves = append(moves, Move{
				FromSquare: fromSquare,
				ToSquare:   toSquare,
				Piece:      piece,
				Flags:      flags,
			})

			bitboard.ClearBit(&attacks, toSquare)
		}

		bitboard.ClearBit(&bb, fromSquare)
	}

	if pos.CastlingRights&(kingsideCastleFlag|queensideCastleFlag) != 0 {
		var kingsideEmptyMask uint64
		var queensideEmptyMask uint64
		var cSquare uint8
		var dSquare uint8
		var eSquare uint8
		var fSquare uint8
		var gSquare uint8
		if isWhite {
			kingsideEmptyMask = constants.CastlingWhiteKingsideEmptyMask
			queensideEmptyMask = constants.CastlingWhiteQueensideEmptyMask
			cSquare = constants.C1
			dSquare = constants.D1
			eSquare = constants.E1
			fSquare = constants.F1
			gSquare = constants.G1
		} else {
			kingsideEmptyMask = constants.CastlingBlackKingsideEmptyMask
			queensideEmptyMask = constants.CastlingBlackQueensideEmptyMask
			cSquare = constants.C8
			dSquare = constants.D8
			eSquare = constants.E8
			fSquare = constants.F8
			gSquare = constants.G8
		}

		if pos.CastlingRights&kingsideCastleFlag != 0 &&
			pos.ColorBitboards[constants.ColorBoth]&kingsideEmptyMask == 0 &&
			!mg.IsSquareAttacked(pos, eSquare, otherColor) &&
			!mg.IsSquareAttacked(pos, fSquare, otherColor) {
			moves = append(moves, Move{
				FromSquare: eSquare,
				ToSquare:   gSquare,
				Piece:      piece,
				Flags:      FlagCastle,
			})
		}

		if pos.CastlingRights&queensideCastleFlag != 0 &&
			pos.ColorBitboards[constants.ColorBoth]&queensideEmptyMask == 0 &&
			!mg.IsSquareAttacked(pos, eSquare, otherColor) &&
			!mg.IsSquareAttacked(pos, dSquare, otherColor) {
			moves = append(moves, Move{
				FromSquare: eSquare,
				ToSquare:   cSquare,
				Piece:      piece,
				Flags:      FlagCastle,
			})
		}
	}

	return moves
}
