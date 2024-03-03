package perft

import (
	"fmt"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"time"
)

func RunPerft(mg *movegen.MoveGen, position *position.Position, depth uint, printInitialMoves bool) {
	fmt.Println("Running PERFT at depth", depth)

	tStart := time.Now()
	total := 0

	if depth == 0 {
		total = 1
	} else {
		moves := mg.GenerateLegalMovesExpensive(position)

		for _, move := range moves {
			nodes := countNodes(mg, &move.Position, depth-1)

			total += nodes

			if printInitialMoves {
				fmt.Printf("%s: %d\n", move.Move.UciString(), nodes)
			}
		}
	}

	tEnd := time.Now()

	fmt.Printf(
		"Depth: %d, Nodes: %d, Time: %v, NPS: %.0f\n",
		depth,
		total,
		tEnd.Sub(tStart),
		float64(total)/tEnd.Sub(tStart).Seconds(),
	)
}

func countNodes(mg *movegen.MoveGen, position *position.Position, depth uint) int {
	if depth == 0 {
		return 1
	}

	total := 0
	moves := mg.GenerateLegalMovesExpensive(position)

	for _, move := range moves {
		total += countNodes(mg, &move.Position, depth-1)
	}

	return total
}
