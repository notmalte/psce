package main

import (
	"errors"
	"fmt"
	"github.com/charmbracelet/huh"
	"github.com/charmbracelet/huh/spinner"
	"github.com/notmalte/psce/internal/move"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"math/rand/v2"
)

func main() {
	var mg *movegen.MoveGen
	initMoveGen := func() {
		mg = movegen.NewMoveGen()
	}

	_ = spinner.
		New().
		Title("Preparing move generator").
		Action(initMoveGen).
		Run()

	pos := position.Initial()
	isUsersTurn := true

	for {
		fmt.Println(pos.PrettyString() + "\n")

		legalMoves := mg.GenerateLegalMoves(pos)
		if len(legalMoves) == 0 {
			fmt.Println("Checkmate or stalemate!")
			return
		}

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

			fmt.Printf("You played: %s\n\n", userMoveUci)
		} else {
			randomMove := legalMoves[rand.IntN(len(legalMoves))]

			pos = pos.MakeMove(mg, &randomMove, false)

			fmt.Printf("Computer played: %s\n\n", randomMove.UciString())
		}

		isUsersTurn = !isUsersTurn
	}
}
