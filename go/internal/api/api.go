package api

import (
	"encoding/json"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/movegen"
	"github.com/notmalte/psce/internal/position"
	"log"
	"net/http"
)

type fenRequest struct {
	Fen string `json:"fen"`
}

type legalMovesResponse struct {
	LegalMoves []string `json:"legalMoves"`
}

type winResponse struct {
	Winner string `json:"winner"`
}

const (
	whiteWins = "white"
	blackWins = "black"
	draw      = "draw"
)

func Run() {
	mg := movegen.NewMoveGen()

	http.HandleFunc("POST /legal-moves", func(w http.ResponseWriter, r *http.Request) {
		var req fenRequest
		err := json.NewDecoder(r.Body).Decode(&req)
		if err != nil {
			http.Error(w, "Invalid request", http.StatusBadRequest)
			return
		}

		if req.Fen == "" {
			http.Error(w, "Invalid request", http.StatusBadRequest)
			return
		}

		pos, err := position.PositionFromFen(req.Fen)
		if err != nil {
			http.Error(w, "Invalid request", http.StatusBadRequest)
			return
		}

		legalMoves := mg.GenerateLegalMoves(pos)

		if len(legalMoves) == 0 {
			whiteKingSquare := pos.GetFirstPieceSquare(constants.WhiteKing)
			blackKingSquare := pos.GetFirstPieceSquare(constants.BlackKing)

			var res winResponse
			if mg.IsSquareAttacked(pos, whiteKingSquare, constants.ColorBlack) {
				res = winResponse{Winner: blackWins}
			} else if mg.IsSquareAttacked(pos, blackKingSquare, constants.ColorWhite) {
				res = winResponse{Winner: whiteWins}
			} else {
				res = winResponse{Winner: draw}
			}

			w.Header().Set("Content-Type", "application/json")
			_ = json.NewEncoder(w).Encode(res)
			return
		}

		legalMovesUci := make([]string, len(legalMoves))
		for i, move := range legalMoves {
			legalMovesUci[i] = move.UciString()
		}

		res := legalMovesResponse{LegalMoves: legalMovesUci}

		w.Header().Set("Content-Type", "application/json")
		_ = json.NewEncoder(w).Encode(res)
	})

	log.Fatal(http.ListenAndServe(":8080", nil))
}
