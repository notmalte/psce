package eval

import (
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/helpers"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
)

func EvaluatePosition(pos *position.Position) int {
	score := 0

	for piece := range constants.PiecesCount {
		bb := pos.PieceBitboards[piece]
		for bb != 0 {
			square := uint8(bits.TrailingZeros64(bb))

			score += materialScore(piece) + squareScore(piece, square)

			bitboard.ClearBit(&bb, square)
		}
	}

	if pos.ColorToMove == constants.ColorWhite {
		return score
	} else {
		return -score
	}
}

func materialScore(piece uint8) int {
	switch piece {
	case constants.WhitePawn:
		return 100
	case constants.WhiteKnight:
		return 320
	case constants.WhiteBishop:
		return 330
	case constants.WhiteRook:
		return 500
	case constants.WhiteQueen:
		return 900
	case constants.WhiteKing:
		return 20000
	case constants.BlackPawn:
		return -100
	case constants.BlackKnight:
		return -320
	case constants.BlackBishop:
		return -330
	case constants.BlackRook:
		return -500
	case constants.BlackQueen:
		return -900
	case constants.BlackKing:
		return -20000
	default:
		panic("invalid piece")
	}
}

func squareScorePawn(square uint8) int {
	return [64]int{
		0, 0, 0, 0, 0, 0, 0, 0,
		50, 50, 50, 50, 50, 50, 50, 50,
		10, 10, 20, 30, 30, 20, 10, 10,
		5, 5, 10, 25, 25, 10, 5, 5,
		0, 0, 0, 20, 20, 0, 0, 0,
		5, -5, -10, 0, 0, -10, -5, 5,
		5, 10, 10, -20, -20, 10, 10, 5,
		0, 0, 0, 0, 0, 0, 0, 0,
	}[square]
}

func squareScoreKnight(square uint8) int {
	return [64]int{
		-50, -40, -30, -30, -30, -30, -40, -50,
		-40, -20, 0, 0, 0, 0, -20, -40,
		-30, 0, 10, 15, 15, 10, 0, -30,
		-30, 5, 15, 20, 20, 15, 5, -30,
		-30, 0, 15, 20, 20, 15, 0, -30,
		-30, 5, 10, 15, 15, 10, 5, -30,
		-40, -20, 0, 5, 5, 0, -20, -40,
		-50, -40, -30, -30, -30, -30, -40, -50,
	}[square]
}

func squareScoreBishop(square uint8) int {
	return [64]int{
		-20, -10, -10, -10, -10, -10, -10, -20,
		-10, 0, 0, 0, 0, 0, 0, -10,
		-10, 0, 5, 10, 10, 5, 0, -10,
		-10, 5, 5, 10, 10, 5, 5, -10,
		-10, 0, 10, 10, 10, 10, 0, -10,
		-10, 10, 10, 10, 10, 10, 10, -10,
		-10, 5, 0, 0, 0, 0, 5, -10,
		-20, -10, -10, -10, -10, -10, -10, -20,
	}[square]
}

func squareScoreRook(square uint8) int {
	return [64]int{
		0, 0, 0, 0, 0, 0, 0, 0,
		5, 10, 10, 10, 10, 10, 10, 5,
		-5, 0, 0, 0, 0, 0, 0, -5,
		-5, 0, 0, 0, 0, 0, 0, -5,
		-5, 0, 0, 0, 0, 0, 0, -5,
		-5, 0, 0, 0, 0, 0, 0, -5,
		-5, 0, 0, 0, 0, 0, 0, -5,
		0, 0, 0, 5, 5, 0, 0, 0,
	}[square]
}

func squareScoreQueen(square uint8) int {
	return [64]int{
		-20, -10, -10, -5, -5, -10, -10, -20,
		-10, 0, 0, 0, 0, 0, 0, -10,
		-10, 0, 5, 5, 5, 5, 0, -10,
		-5, 0, 5, 5, 5, 5, 0, -5,
		0, 0, 5, 5, 5, 5, 0, -5,
		-10, 5, 5, 5, 5, 5, 0, -10,
		-10, 0, 5, 0, 0, 0, 0, -10,
		-20, -10, -10, -5, -5, -10, -10, -20,
	}[square]
}

func squareScore(piece uint8, square uint8) int {
	switch piece {
	case constants.WhitePawn:
		return squareScorePawn(square)
	case constants.WhiteKnight:
		return squareScoreKnight(square)
	case constants.WhiteBishop:
		return squareScoreBishop(square)
	case constants.WhiteRook:
		return squareScoreRook(square)
	case constants.WhiteQueen:
		return squareScoreQueen(square)
	case constants.WhiteKing:
		return 0
	case constants.BlackPawn:
		return -squareScorePawn(helpers.GetMirrorSquare(square))
	case constants.BlackKnight:
		return -squareScoreKnight(helpers.GetMirrorSquare(square))
	case constants.BlackBishop:
		return -squareScoreBishop(helpers.GetMirrorSquare(square))
	case constants.BlackRook:
		return -squareScoreRook(helpers.GetMirrorSquare(square))
	case constants.BlackQueen:
		return -squareScoreQueen(helpers.GetMirrorSquare(square))
	case constants.BlackKing:
		return 0
	default:
		panic("invalid piece")
	}
}
