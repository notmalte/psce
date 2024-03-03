package perft

import (
	"fmt"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"time"
)

func RunPerft(mg *movegen.MoveGen, position *position.Position, depth uint) {
	fmt.Println("Running PERFT at depth", depth)

	tStart := time.Now()
	total := countNodes(mg, position, depth)

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
	nextPositions := mg.GenerateLegalNextPositions(position)

	for _, nextPosition := range nextPositions {
		total += countNodes(mg, nextPosition, depth-1)
	}

	return total
}
