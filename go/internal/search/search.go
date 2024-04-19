package search

import (
	"fmt"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/eval"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"github.com/notmalte/psce/internal/tt"
	"github.com/notmalte/psce/internal/zobrist"
	"math/bits"
	"slices"
	"time"
)

const MaxSearchDepth = 64

type Context struct {
	MoveGen     *movegen.MoveGen
	ZobristKeys *zobrist.Keys
}

type killerMovesArray [MaxSearchDepth][2]*move.Move

type sortingHeuristics struct {
	killerMoves *killerMovesArray
	prevPv      []*move.Move
}

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

func generateSortedPseudoLegalMoves(mg *movegen.MoveGen, pos *position.Position, sh *sortingHeuristics, ply int) []move.Move {
	pseudoMoves := mg.GeneratePseudoLegalMoves(pos)

	slices.SortFunc(pseudoMoves, func(a, b move.Move) int {
		if sh != nil {
			if sh.prevPv != nil && ply < len(sh.prevPv) {
				pvMove := sh.prevPv[ply]

				if a == *pvMove {
					return -1
				}

				if b == *pvMove {
					return 1
				}
			}

			if sh.killerMoves != nil {
				aKiller := (sh.killerMoves[ply][0] != nil && a == *sh.killerMoves[ply][0]) || (sh.killerMoves[ply][1] != nil && a == *sh.killerMoves[ply][1])
				bKiller := (sh.killerMoves[ply][0] != nil && b == *sh.killerMoves[ply][0]) || (sh.killerMoves[ply][1] != nil && b == *sh.killerMoves[ply][1])

				if aKiller && !bKiller {
					return -1
				}

				if !aKiller && bKiller {
					return 1
				}
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

func quiescence(ctx *Context, pos *position.Position, alpha int, beta int) int {
	ev := eval.EvaluatePosition(pos)

	if ev >= beta {
		return beta
	}

	if ev > alpha {
		alpha = ev
	}

	pseudoMoves := generateSortedPseudoLegalMoves(ctx.MoveGen, pos, nil, 0)

	for _, pseudoMove := range pseudoMoves {
		newPos := pos.MakeMove(ctx.MoveGen, &pseudoMove, true)

		if newPos != nil {
			ev := -quiescence(ctx, newPos, -beta, -alpha)

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

func negamax(ttHit *uint, ttMiss *uint, ctx *Context, pos *position.Position, sh *sortingHeuristics, table *tt.TranspositionTable, depth uint, alpha int, beta int, ply int) (int, []*move.Move) {
	hash := ctx.ZobristKeys.GenerateHash(pos)

	score, found := table.Probe(hash, depth, alpha, beta)
	if found && ply > 0 {
		*ttHit++
		return score, []*move.Move{}
	} else {
		*ttMiss++
	}

	if depth == 0 {
		qs := quiescence(ctx, pos, alpha, beta)
		table.Store(hash, depth, tt.FlagExact, qs)
		return qs, []*move.Move{}
	}

	pseudoMoves := generateSortedPseudoLegalMoves(ctx.MoveGen, pos, sh, ply)

	canMove := false

	var pv []*move.Move

	ttFlag := tt.FlagAlpha

	for _, pseudoMove := range pseudoMoves {
		newPos := pos.MakeMove(ctx.MoveGen, &pseudoMove, false)

		if newPos != nil {
			canMove = true
			evNeg, childPv := negamax(ttHit, ttMiss, ctx, newPos, sh, table, depth-1, -beta, -alpha, ply+1)

			ev := -evNeg

			if ev >= beta {
				if !pseudoMove.IsCapture() &&
					(sh.killerMoves[ply][0] == nil || pseudoMove != *sh.killerMoves[ply][0]) &&
					(sh.killerMoves[ply][1] == nil || pseudoMove != *sh.killerMoves[ply][1]) {
					sh.killerMoves[ply][1] = sh.killerMoves[ply][0]
					sh.killerMoves[ply][0] = &pseudoMove
				}

				table.Store(hash, depth, tt.FlagBeta, beta)
				return beta, []*move.Move{&pseudoMove}
			}

			if ev > alpha {
				ttFlag = tt.FlagExact

				alpha = ev
				pv = append([]*move.Move{&pseudoMove}, childPv...)
			}
		}
	}

	if canMove {
		table.Store(hash, depth, ttFlag, alpha)
		return alpha, pv
	}

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

	if ctx.MoveGen.IsSquareAttacked(pos, kingSquare, opponentColor) {
		return -eval.CheckmateScore + ply, []*move.Move{}
	} else {
		return 0, []*move.Move{}
	}
}

func Search(ctx *Context, pos *position.Position, table *tt.TranspositionTable, minSearchDuration time.Duration) (int, *move.Move, []*move.Move) {
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

		sh := &sortingHeuristics{
			killerMoves: killerMoves,
			prevPv:      pv,
		}

		ttHit := uint(0)
		ttMiss := uint(0)

		score, pv = negamax(&ttHit, &ttMiss, ctx, pos, sh, table, depth, -eval.CheckmateScore, eval.CheckmateScore, 0)

		fmt.Printf("TT hit rate: %.2f%% (%d/%d)\n", float64(ttHit)/float64(ttHit+ttMiss)*100, ttHit, ttHit+ttMiss)

		if time.Since(tStart) >= minSearchDuration {
			break
		}

		if score >= eval.CheckmateScore-MaxSearchDepth {
			break
		}
	}

	return score, pv[0], pv
}
