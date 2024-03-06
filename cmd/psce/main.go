package main

import (
	"errors"
	"fmt"
	"github.com/charmbracelet/huh"
	"github.com/charmbracelet/huh/spinner"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"github.com/notmalte/psce/internal/search"
	"math/bits"
	"time"
)

func main() {
	var mg *movegen.MoveGen
	initMoveGen := func() {
		mg = movegen.NewMoveGen()
	}

	_ = spinner.
		New().
		Title("Preparing move generator...").
		Action(initMoveGen).
		Run()

	pos := position.Initial()
	isUsersTurn := true

	fmt.Println(pos.PrettyString(nil, constants.NoSquare) + "\n")

	for {
		legalMoves := mg.GenerateLegalMoves(pos)
		if len(legalMoves) == 0 {
			whiteKingSquare := uint8(bits.TrailingZeros64(pos.PieceBitboards[constants.WhiteKing]))
			blackKingSquare := uint8(bits.TrailingZeros64(pos.PieceBitboards[constants.BlackKing]))

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

			var userMove *move.Move
			for _, legalMove := range legalMoves {
				if userMoveUci == legalMove.UciString() {
					userMove = &legalMove
					break
				}
			}

			pos = pos.MakeMove(mg, userMove, false)

			moveFrom = userMove.FromSquare
			moveTo = userMove.ToSquare

			fmt.Printf("You played: %s\n\n", userMoveUci)
		} else {
			var bestScore int
			var bestMove *move.Move
			var ms int64

			findBestMove := func() {
				tStart := time.Now()
				bestScore, bestMove = search.Search(mg, pos, 6)
				ms = time.Since(tStart).Milliseconds()
			}

			_ = spinner.
				New().
				Title("Thinking...").
				Action(findBestMove).
				Run()

			pos = pos.MakeMove(mg, bestMove, false)

			moveFrom = bestMove.FromSquare
			moveTo = bestMove.ToSquare

			fmt.Printf("Computer played: %s (score: %d, took: %dms)\n\n", bestMove.UciString(), bestScore, ms)
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

		kingSquare := uint8(bits.TrailingZeros64(pos.PieceBitboards[kingPiece]))
		if mg.IsSquareAttacked(pos, kingSquare, opponentColor) {
			checkSquare = kingSquare
		}

		fmt.Println(pos.PrettyString([]uint8{moveFrom, moveTo}, checkSquare) + "\n")

		isUsersTurn = !isUsersTurn
	}
}
