package position

import (
	"errors"
	"fmt"
	"github.com/charmbracelet/lipgloss"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/helpers"
	"github.com/notmalte/psce/internal/move"
	"math/bits"
	"strings"
)

type Position struct {
	PieceBitboards  [12]uint64
	ColorBitboards  [3]uint64
	EnPassantSquare uint8
	CastlingRights  uint8
	ColorToMove     uint8
}

func Empty() *Position {
	pos := &Position{}
	pos.PieceBitboards = [12]uint64{}
	pos.ColorBitboards = [3]uint64{}
	pos.EnPassantSquare = constants.NoSquare
	pos.CastlingRights = constants.CastlingNone
	pos.ColorToMove = constants.ColorWhite

	return pos
}

func (pos *Position) String() string {
	s := ""

	for row := range uint8(8) {
		s += fmt.Sprintf("%d ", 8-row)
	outer:
		for col := range uint8(8) {
			index := bitboard.RowColToIndex(row, col)

			for piece := range constants.PiecesCount {
				if bitboard.GetBit(pos.PieceBitboards[piece], index) {
					s += fmt.Sprintf("%s ", helpers.PieceString(piece))
					continue outer
				}
			}

			s += ". "
		}
		s += "\n"
	}

	s += "  a b c d e f g h\n\n"

	s += fmt.Sprintf("Color to move: %s\n", helpers.ColorString(pos.ColorToMove))
	if pos.EnPassantSquare != constants.NoSquare {
		s += fmt.Sprintf("En passant square: %s\n", helpers.SquareString(pos.EnPassantSquare))
	} else {
		s += "En passant square: -\n"
	}
	s += fmt.Sprintf("Castling rights: %s", helpers.CastlingString(pos.CastlingRights))

	return s
}

func (pos *Position) PrettyString(highlightSquares []uint8, checkSquare uint8) string {
	infoStyle := lipgloss.NewStyle().Foreground(lipgloss.Color("#7f7f7f"))
	brightSquareStyle := lipgloss.NewStyle().Background(lipgloss.Color("#d9d9d9")).Foreground(lipgloss.Color("#000000"))
	darkSquareStyle := lipgloss.NewStyle().Background(lipgloss.Color("#8c8c8c")).Foreground(lipgloss.Color("#000000"))
	brightHighlightSquareStyle := lipgloss.NewStyle().Background(lipgloss.Color("#c6e48b")).Foreground(lipgloss.Color("#000000"))
	darkHighlightSquareStyle := lipgloss.NewStyle().Background(lipgloss.Color("#98b06a")).Foreground(lipgloss.Color("#000000"))
	checkSquareStyle := lipgloss.NewStyle().Background(lipgloss.Color("#bf6767")).Foreground(lipgloss.Color("#000000"))

	s := ""

	for row := range uint8(8) {
		s += infoStyle.Render(fmt.Sprintf("%d ", 8-row))
	outer:
		for col := range uint8(8) {
			index := bitboard.RowColToIndex(row, col)
			var style lipgloss.Style

			if checkSquare == index {
				style = checkSquareStyle
			} else {
				isHighlighted := false

				for _, square := range highlightSquares {
					if square == index {
						isHighlighted = true
						break
					}
				}

				if isHighlighted {
					if (row+col)%2 == 0 {
						style = brightHighlightSquareStyle
					} else {
						style = darkHighlightSquareStyle
					}
				} else {
					if (row+col)%2 == 0 {
						style = brightSquareStyle
					} else {
						style = darkSquareStyle
					}
				}
			}

			for piece := range constants.PiecesCount {
				if bitboard.GetBit(pos.PieceBitboards[piece], index) {
					s += style.Render(fmt.Sprintf(" %s ", helpers.PieceStringUnicode(piece)))
					continue outer
				}
			}

			s += style.Render("   ")
		}
		s += "\n"
	}

	s += infoStyle.Render("   a  b  c  d  e  f  g  h") + "\n\n"

	s += infoStyle.Render(fmt.Sprintf("Color to move: %s", helpers.ColorString(pos.ColorToMove))) + "\n"
	if pos.EnPassantSquare != constants.NoSquare {
		s += infoStyle.Render(fmt.Sprintf("En passant square: %s", helpers.SquareString(pos.EnPassantSquare))) + "\n"
	} else {
		s += infoStyle.Render("En passant square: -") + "\n"
	}
	s += infoStyle.Render(fmt.Sprintf("Castling rights: %s", helpers.CastlingString(pos.CastlingRights)))

	return s
}

var ErrInvalidFen = errors.New("invalid FEN")

func PositionFromFen(fen string) (*Position, error) {
	pos := Empty()

	fenParts := strings.Split(fen, " ")
	if len(fenParts) < 4 {
		return nil, ErrInvalidFen
	}

	rows := strings.Split(fenParts[0], "/")
	if len(rows) != 8 {
		return nil, ErrInvalidFen
	}

	for row, rowString := range rows {
		col := 0

		for _, c := range rowString {
			if c >= '1' && c <= '8' {
				col += int(c - '0')
			} else {
				index := bitboard.RowColToIndex(uint8(row), uint8(col))
				piece, err := helpers.StringToPiece(string(c))
				if err != nil {
					return nil, ErrInvalidFen
				}
				color := helpers.PieceColor(piece)

				bitboard.SetBit(&pos.PieceBitboards[piece], index)
				bitboard.SetBit(&pos.ColorBitboards[color], index)

				col++
			}
		}

		if col != 8 {
			return nil, ErrInvalidFen
		}
	}

	pos.ColorBitboards[constants.ColorBoth] = pos.ColorBitboards[constants.ColorWhite] | pos.ColorBitboards[constants.ColorBlack]

	if fenParts[1] == "w" {
		pos.ColorToMove = constants.ColorWhite
	} else if fenParts[1] == "b" {
		pos.ColorToMove = constants.ColorBlack
	} else {
		return nil, ErrInvalidFen
	}

	pos.CastlingRights = helpers.StringToCastling(fenParts[2])

	if fenParts[3] != "-" {
		square, err := helpers.StringToSquare(fenParts[3])
		if err != nil {
			return nil, ErrInvalidFen
		}
		pos.EnPassantSquare = square
	}

	return pos, nil
}

func Initial() *Position {
	pos, _ := PositionFromFen(constants.InitialPositionFEN)
	return pos
}

func (pos *Position) applyPseudoLegalMove(move *move.Move) {
	isWhite := pos.ColorToMove == constants.ColorWhite

	var opponentColor uint8
	if isWhite {
		opponentColor = constants.ColorBlack
	} else {
		opponentColor = constants.ColorWhite
	}

	bitboard.ClearBit(&pos.PieceBitboards[move.Piece], move.FromSquare)
	bitboard.ClearBit(&pos.ColorBitboards[pos.ColorToMove], move.FromSquare)

	if move.IsPromotion() {
		bitboard.SetBit(&pos.PieceBitboards[move.PromotionPiece], move.ToSquare)
	} else {
		bitboard.SetBit(&pos.PieceBitboards[move.Piece], move.ToSquare)
	}
	bitboard.SetBit(&pos.ColorBitboards[pos.ColorToMove], move.ToSquare)

	if move.IsCapture() {
		if move.IsEnPassant() {
			var capturedPawnSquare uint8
			var capturedPawnPiece uint8
			if isWhite {
				capturedPawnSquare = move.ToSquare + 8
				capturedPawnPiece = constants.BlackPawn
			} else {
				capturedPawnSquare = move.ToSquare - 8
				capturedPawnPiece = constants.WhitePawn
			}

			bitboard.ClearBit(&pos.PieceBitboards[capturedPawnPiece], capturedPawnSquare)
			bitboard.ClearBit(&pos.ColorBitboards[opponentColor], capturedPawnSquare)
		} else {
			var opponentPieces [6]uint8
			if isWhite {
				opponentPieces = [6]uint8{constants.BlackPawn, constants.BlackKnight, constants.BlackBishop, constants.BlackRook, constants.BlackQueen, constants.BlackKing}
			} else {
				opponentPieces = [6]uint8{constants.WhitePawn, constants.WhiteKnight, constants.WhiteBishop, constants.WhiteRook, constants.WhiteQueen, constants.WhiteKing}
			}

			for _, piece := range opponentPieces {
				bitboard.ClearBit(&pos.PieceBitboards[piece], move.ToSquare)
			}
			bitboard.ClearBit(&pos.ColorBitboards[opponentColor], move.ToSquare)

			castlingMask := uint8(0)
			switch move.ToSquare {
			case constants.A8:
				castlingMask = constants.CastlingBlackQueenside
			case constants.H8:
				castlingMask = constants.CastlingBlackKingside
			case constants.A1:
				castlingMask = constants.CastlingWhiteQueenside
			case constants.H1:
				castlingMask = constants.CastlingWhiteKingside
			default:
			}

			pos.CastlingRights &= ^castlingMask
		}
	}

	if move.IsDoublePawnPush() {
		if isWhite {
			pos.EnPassantSquare = move.ToSquare + 8
		} else {
			pos.EnPassantSquare = move.ToSquare - 8
		}
	} else {
		pos.EnPassantSquare = constants.NoSquare
	}

	if move.IsCastle() {
		var rookFromSquare uint8
		var rookToSquare uint8
		switch move.ToSquare {
		case constants.C8:
			rookFromSquare, rookToSquare = constants.A8, constants.D8
		case constants.G8:
			rookFromSquare, rookToSquare = constants.H8, constants.F8
		case constants.C1:
			rookFromSquare, rookToSquare = constants.A1, constants.D1
		case constants.G1:
			rookFromSquare, rookToSquare = constants.H1, constants.F1
		default:
			panic("Invalid castling move")
		}

		var rookPiece uint8
		if isWhite {
			rookPiece = constants.WhiteRook
		} else {
			rookPiece = constants.BlackRook
		}

		bitboard.ClearBit(&pos.PieceBitboards[rookPiece], rookFromSquare)
		bitboard.ClearBit(&pos.ColorBitboards[pos.ColorToMove], rookFromSquare)

		bitboard.SetBit(&pos.PieceBitboards[rookPiece], rookToSquare)
		bitboard.SetBit(&pos.ColorBitboards[pos.ColorToMove], rookToSquare)

		if isWhite {
			pos.CastlingRights &= ^(constants.CastlingWhiteKingside | constants.CastlingWhiteQueenside)
		} else {
			pos.CastlingRights &= ^(constants.CastlingBlackKingside | constants.CastlingBlackQueenside)
		}
	} else {
		castlingMask := uint8(0)
		switch move.FromSquare {
		case constants.A8:
			castlingMask = constants.CastlingBlackQueenside
		case constants.E8:
			castlingMask = constants.CastlingBlackKingside | constants.CastlingBlackQueenside
		case constants.H8:
			castlingMask = constants.CastlingBlackKingside
		case constants.A1:
			castlingMask = constants.CastlingWhiteQueenside
		case constants.E1:
			castlingMask = constants.CastlingWhiteKingside | constants.CastlingWhiteQueenside
		case constants.H1:
			castlingMask = constants.CastlingWhiteKingside
		default:
		}

		pos.CastlingRights &= ^castlingMask
	}

	pos.ColorBitboards[constants.ColorBoth] = pos.ColorBitboards[constants.ColorWhite] | pos.ColorBitboards[constants.ColorBlack]
	pos.ColorToMove = opponentColor
}

// just to prevent cyclic imports
type moveValidator interface {
	IsSquareAttacked(pos *Position, square uint8, attackerColor uint8) bool
}

func (pos *Position) MakeMove(mv moveValidator, move *move.Move, onlyCaptures bool) *Position {
	if onlyCaptures && !move.IsCapture() {
		return nil
	}

	clone := *pos
	clone.applyPseudoLegalMove(move)

	var kingPiece uint8
	if pos.ColorToMove == constants.ColorWhite {
		kingPiece = constants.WhiteKing
	} else {
		kingPiece = constants.BlackKing
	}

	kingSquare := uint8(bits.TrailingZeros64(clone.PieceBitboards[kingPiece]))

	if mv.IsSquareAttacked(&clone, kingSquare, clone.ColorToMove) {
		return nil
	}

	return &clone
}

func (pos *Position) GetMoveVictimPiece(m *move.Move) uint8 {
	if m.IsEnPassant() {
		if m.Piece == constants.WhitePawn {
			return constants.BlackPawn
		} else {
			return constants.WhitePawn
		}
	}

	if m.IsCapture() {
		for piece := range constants.PiecesCount {
			if bitboard.GetBit(pos.PieceBitboards[piece], m.ToSquare) {
				return piece
			}
		}
	}

	panic("no victim piece")
}

func (pos *Position) GetFirstPieceSquare(piece uint8) uint8 {
	return uint8(bits.TrailingZeros64(pos.PieceBitboards[piece]))
}
