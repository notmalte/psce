package interactive

import (
	"errors"
	"fmt"
	"github.com/charmbracelet/huh"
	"github.com/charmbracelet/huh/spinner"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/eval"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"github.com/notmalte/psce/internal/search"
	"github.com/notmalte/psce/internal/tt"
	"github.com/notmalte/psce/internal/zobrist"
	"time"
)

func Run() {
	var userColor uint8
	err := huh.
		NewSelect[uint8]().
		Title("Choose your color").
		Options(
			huh.NewOption("White", constants.ColorWhite),
			huh.NewOption("Black", constants.ColorBlack),
		).
		Value(&userColor).
		Run()

	if err != nil {
		if errors.Is(err, huh.ErrUserAborted) {
			fmt.Println("Exiting...")
			return
		}

		panic(err)
	}

	var mg *movegen.MoveGen
	var zk *zobrist.Keys
	var ctx *search.Context
	initMoveGenAndZobrist := func() {
		mg = movegen.NewMoveGen()
		zk = zobrist.NewKeys()
		ctx = &search.Context{
			MoveGen:     mg,
			ZobristKeys: zk,
		}
	}

	_ = spinner.
		New().
		Title("Preparing move generator and Zobrist keys...").
		Action(initMoveGenAndZobrist).
		Run()

	pos := position.Initial()
	hash := zk.GenerateHash(pos)

	table := tt.NewTranspositionTable(1 << 22)

	isUsersTurn := userColor == pos.ColorToMove

	type historyEntry struct {
		posBeforeMove *position.Position
		move          *move.Move
	}

	history := []historyEntry{}

	fmt.Println(pos.PrettyString(nil, constants.NoSquare) + "\n")

	for {
		legalMoves := mg.GenerateLegalMoves(pos)
		if len(legalMoves) == 0 {
			whiteKingSquare := pos.GetFirstPieceSquare(constants.WhiteKing)
			blackKingSquare := pos.GetFirstPieceSquare(constants.BlackKing)

			if mg.IsSquareAttacked(pos, whiteKingSquare, constants.ColorBlack) {
				fmt.Println("Black wins by checkmate!")
			} else if mg.IsSquareAttacked(pos, blackKingSquare, constants.ColorWhite) {
				fmt.Println("White wins by checkmate!")
			} else {
				fmt.Println("Draw by stalemate!")
			}

			return
		}

		var moveFrom, moveTo uint8

		if isUsersTurn {
			validateInput := func(input string) error {
				if input == "takeback" {
					if len(history) < 2 {
						return errors.New("no moves to take back")
					}

					return nil
				}

				for _, legalMove := range legalMoves {
					if input == legalMove.UciString() {
						return nil
					}
				}

				return errors.New("invalid move")
			}

			var userMoveUci string

			err := huh.
				NewInput().
				Title("Enter your move").
				Value(&userMoveUci).
				Validate(validateInput).
				Run()

			if err != nil {
				if errors.Is(err, huh.ErrUserAborted) {
					fmt.Println("Exiting...")
					return
				}

				panic(err)
			}

			if userMoveUci == "takeback" {
				pos = history[len(history)-2].posBeforeMove

				if len(history) > 2 {
					opponentMove := history[len(history)-3].move

					moveFrom = opponentMove.FromSquare
					moveTo = opponentMove.ToSquare

					fmt.Printf("Took back move - last opponent move was: %s\n\n", opponentMove.UciString())
				} else {
					moveFrom = constants.NoSquare
					moveTo = constants.NoSquare

					fmt.Printf("Took back move\n\n")
				}

				isUsersTurn = !isUsersTurn

				history = history[:len(history)-2]
			} else {
				var userMove *move.Move
				for _, legalMove := range legalMoves {
					if userMoveUci == legalMove.UciString() {
						userMove = &legalMove
						break
					}
				}

				history = append(history, historyEntry{posBeforeMove: pos, move: userMove})
				newPos := pos.MakeMove(mg, userMove, false)

				incrHash := zk.IncrementalHash(hash, pos, newPos, userMove)
				freshHash := zk.GenerateHash(newPos)

				if freshHash != incrHash {
					panic("Hash mismatch")
				}

				hash = incrHash
				pos = newPos

				moveFrom = userMove.FromSquare
				moveTo = userMove.ToSquare

				fmt.Printf("You played: %s\n\n", userMoveUci)
			}
		} else {
			var bestScore int
			var bestMove *move.Move
			var bestPv []*move.Move

			var ms int64

			findBestMove := func() {
				tStart := time.Now()
				bestScore, bestMove, bestPv = search.Search(ctx, pos, table, 5*time.Second)
				ms = time.Since(tStart).Milliseconds()
			}

			_ = spinner.
				New().
				Title("Thinking...").
				Action(findBestMove).
				Run()

			history = append(history, historyEntry{posBeforeMove: pos, move: bestMove})
			newPos := pos.MakeMove(mg, bestMove, false)

			fmt.Printf("Previous hash: %d\n", hash)

			t0Incr := time.Now()
			incrHash := zk.IncrementalHash(hash, pos, newPos, bestMove)
			fmt.Printf("Incremental hash: %d (took: %dns)\n", incrHash, time.Since(t0Incr).Nanoseconds())

			t0Fresh := time.Now()
			freshHash := zk.GenerateHash(newPos)
			fmt.Printf("Fresh hash: %d (took: %dns)\n", freshHash, time.Since(t0Fresh).Nanoseconds())

			if freshHash != incrHash {
				panic("Hash mismatch")
			}

			hash = incrHash
			pos = newPos

			moveFrom = bestMove.FromSquare
			moveTo = bestMove.ToSquare

			fmt.Printf("Computer played: %s (score: %d, took: %dms)\n", bestMove.UciString(), bestScore, ms)
			if len(bestPv) != 0 {
				fmt.Printf("Principal variation (depth %d):", len(bestPv))
				for _, pvMove := range bestPv {
					fmt.Printf(" %s", pvMove.UciString())
				}
				fmt.Println()
			}

			if bestScore+search.MaxSearchDepth >= eval.CheckmateScore {
				fmt.Printf("Computer expects mate in %d move(s)\n", eval.CheckmateScore-bestScore)
			}

			fmt.Println()
		}

		var opponentColor uint8
		var kingPiece uint8
		if pos.ColorToMove == constants.ColorWhite {
			opponentColor = constants.ColorBlack
			kingPiece = constants.WhiteKing
		} else {
			opponentColor = constants.ColorWhite
			kingPiece = constants.BlackKing
		}

		checkSquare := constants.NoSquare

		kingSquare := pos.GetFirstPieceSquare(kingPiece)
		if mg.IsSquareAttacked(pos, kingSquare, opponentColor) {
			checkSquare = kingSquare
		}

		fmt.Println(pos.PrettyString([]uint8{moveFrom, moveTo}, checkSquare) + "\n")

		isUsersTurn = !isUsersTurn
	}
}
