package search

import (
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/eval"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
)

func quiescence(mg *movegen.MoveGen, pos *position.Position, alpha int, beta int) int {
	ev := eval.EvaluatePosition(pos)

	if ev >= beta {
		return beta
	}

	if ev > alpha {
		alpha = ev
	}

	pseudoMoves := mg.GeneratePseudoLegalMoves(pos)

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

	pseudoMoves := mg.GeneratePseudoLegalMoves(pos)

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
	pseudoMoves := mg.GeneratePseudoLegalMoves(pos)

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
