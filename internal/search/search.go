package search

import (
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/eval"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"math/bits"
	"slices"
	"time"
)

const MaxSearchDepth = 64

type killerMovesArray [MaxSearchDepth][2]*move.Move

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

func generateSortedPseudoLegalMoves(mg *movegen.MoveGen, pos *position.Position, ply int, killerMoves *killerMovesArray, prevPv []*move.Move) []move.Move {
	pseudoMoves := mg.GeneratePseudoLegalMoves(pos)

	slices.SortFunc(pseudoMoves, func(a, b move.Move) int {
		if prevPv != nil && ply < len(prevPv) {
			pvMove := prevPv[ply]

			if a == *pvMove {
				return -1
			}

			if b == *pvMove {
				return 1
			}
		}

		if killerMoves != nil {
			aKiller := (killerMoves[ply][0] != nil && a == *killerMoves[ply][0]) || (killerMoves[ply][1] != nil && a == *killerMoves[ply][1])
			bKiller := (killerMoves[ply][0] != nil && b == *killerMoves[ply][0]) || (killerMoves[ply][1] != nil && b == *killerMoves[ply][1])

			if aKiller && !bKiller {
				return -1
			}

			if !aKiller && bKiller {
				return 1
			}
		}

		aCapture := a.IsCapture()
		bCapture := b.IsCapture()

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

	pseudoMoves := generateSortedPseudoLegalMoves(mg, pos, 0, nil, nil)

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

func negamax(mg *movegen.MoveGen, pos *position.Position, depth uint, alpha int, beta int, ply int, killerMoves *killerMovesArray, prevPv []*move.Move) (int, []*move.Move) {
	if depth == 0 {
		return quiescence(mg, pos, alpha, beta), []*move.Move{}
	}

	pseudoMoves := generateSortedPseudoLegalMoves(mg, pos, ply, killerMoves, prevPv)

	canMove := false

	var pv []*move.Move

	for _, pseudoMove := range pseudoMoves {
		newPos := pos.MakeMove(mg, &pseudoMove, false)

		if newPos != nil {
			canMove = true
			evNeg, childPv := negamax(mg, newPos, depth-1, -beta, -alpha, ply+1, killerMoves, prevPv)

			ev := -evNeg

			if ev >= beta {
				if !pseudoMove.IsCapture() &&
					(killerMoves[ply][0] == nil || pseudoMove != *killerMoves[ply][0]) &&
					(killerMoves[ply][1] == nil || pseudoMove != *killerMoves[ply][1]) {
					killerMoves[ply][1] = killerMoves[ply][0]
					killerMoves[ply][0] = &pseudoMove
				}

				return beta, []*move.Move{&pseudoMove}
			}

			if ev > alpha {
				alpha = ev
				pv = append([]*move.Move{&pseudoMove}, childPv...)
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
			return -eval.CheckmateScore + ply, []*move.Move{}
		} else {
			return 0, []*move.Move{}
		}
	}

	return alpha, pv
}

func Search(mg *movegen.MoveGen, pos *position.Position, minSearchDuration time.Duration) (int, *move.Move, []*move.Move) {
	if minSearchDuration <= 0 {
		panic("min search duration must be positive")
	}

	tStart := time.Now()

	var score int
	var pv []*move.Move

	for depth := uint(1); depth <= MaxSearchDepth; depth++ {
		killerMoves := &killerMovesArray{}
		for i := range killerMoves {
			killerMoves[i][0], killerMoves[i][1] = nil, nil
		}

		score, pv = negamax(mg, pos, depth, -eval.CheckmateScore, eval.CheckmateScore, 0, killerMoves, pv)

		if time.Since(tStart) >= minSearchDuration {
			break
		}

		if score >= eval.CheckmateScore-MaxSearchDepth {
			break
		}
	}

	return score, pv[0], pv
}
