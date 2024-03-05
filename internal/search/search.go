package search

import (
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/eval"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
	"slices"
)

func getPieceValueForSort(piece uint8) int {
	switch piece {
	case constants.WhitePawn, constants.BlackPawn:
		return 1
	case constants.WhiteKnight, constants.BlackKnight:
		return 2
	case constants.WhiteBishop, constants.BlackBishop:
		return 3
	case constants.WhiteRook, constants.BlackRook:
		return 4
	case constants.WhiteQueen, constants.BlackQueen:
		return 5
	case constants.WhiteKing, constants.BlackKing:
		return 6
	default:
		panic("invalid piece")
	}
}

func generateSortedPseudoLegalMoves(mg *movegen.MoveGen, pos *position.Position) []move.Move {
	pseudoMoves := mg.GeneratePseudoLegalMoves(pos)

	slices.SortFunc(pseudoMoves, func(a, b move.Move) int {
		aCapture := a.HasFlag(constants.MoveFlagCapture)
		bCapture := b.HasFlag(constants.MoveFlagCapture)

		if !aCapture && !bCapture {
			return 0
		}

		if !aCapture && bCapture {
			return 1
		}

		if aCapture && !bCapture {
			return -1
		}

		aVictimValue := getPieceValueForSort(pos.GetMoveVictimPiece(&a))
		bVictimValue := getPieceValueForSort(pos.GetMoveVictimPiece(&b))

		if aVictimValue != bVictimValue {
			return bVictimValue - aVictimValue
		}

		aPieceValue := getPieceValueForSort(a.Piece)
		bPieceValue := getPieceValueForSort(b.Piece)

		return aPieceValue - bPieceValue
	})

	return pseudoMoves
}

func quiescence(mg *movegen.MoveGen, pos *position.Position, alpha int, beta int) int {
	ev := eval.EvaluatePosition(pos)

	if ev >= beta {
		return beta
	}

	if ev > alpha {
		alpha = ev
	}

	pseudoMoves := generateSortedPseudoLegalMoves(mg, pos)

	for _, pseudoMove := range pseudoMoves {
		newPos := pos.MakeMove(mg, &pseudoMove, true)

		if newPos != nil {
			ev := -quiescence(mg, newPos, -beta, -alpha)

			if ev >= beta {
				return beta
			}

			if ev > alpha {
				alpha = ev
			}
		}
	}

	return alpha
}

func negamax(mg *movegen.MoveGen, pos *position.Position, depth uint, alpha int, beta int, ply int) int {
	if depth == 0 {
		return quiescence(mg, pos, alpha, beta)
	}

	pseudoMoves := generateSortedPseudoLegalMoves(mg, pos)

	canMove := false

	for _, pseudoMove := range pseudoMoves {
		newPos := pos.MakeMove(mg, &pseudoMove, false)

		if newPos != nil {
			canMove = true
			ev := -negamax(mg, newPos, depth-1, -beta, -alpha, ply+1)

			if ev >= beta {
				return beta
			}

			if ev > alpha {
				alpha = ev
			}
		}
	}

	if !canMove {
		isWhite := pos.ColorToMove == constants.ColorWhite

		var opponentColor uint8
		var kingPiece uint8
		if isWhite {
			opponentColor = constants.ColorBlack
			kingPiece = constants.WhiteKing
		} else {
			opponentColor = constants.ColorWhite
			kingPiece = constants.BlackKing
		}

		kingSquare := uint8(bits.TrailingZeros64(pos.PieceBitboards[kingPiece]))

		if mg.IsSquareAttacked(pos, kingSquare, opponentColor) {
			return -eval.CheckmateScore + ply
		} else {
			return 0
		}
	}

	return alpha
}

func Search(mg *movegen.MoveGen, pos *position.Position, depth uint) (int, *move.Move) {
	pseudoMoves := generateSortedPseudoLegalMoves(mg, pos)

	bestScore := -eval.CheckmateScore
	var bestMove *move.Move

	for _, pseudoMove := range pseudoMoves {
		newPos := pos.MakeMove(mg, &pseudoMove, false)

		if newPos != nil {
			ev := -negamax(mg, newPos, depth-1, bestScore, eval.CheckmateScore, 1)

			if ev > bestScore {
				bestScore = ev
				bestMove = &pseudoMove
			}
		}
	}

	return bestScore, bestMove
}
