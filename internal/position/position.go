package position

import (
	"errors"
	"fmt"
	"github.com/notmalte/psce/internal/bitboard"
	"github.com/notmalte/psce/internal/constants"
	"github.com/notmalte/psce/internal/helpers"
	"github.com/notmalte/psce/internal/move"
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

func (pos *Position) ApplyPseudoLegalMove(move *move.Move) {
	isWhite := pos.ColorToMove == constants.ColorWhite

	var opponentColor uint8
	if isWhite {
		opponentColor = constants.ColorBlack
	} else {
		opponentColor = constants.ColorWhite
	}

	bitboard.ClearBit(&pos.PieceBitboards[move.Piece], move.FromSquare)
	bitboard.ClearBit(&pos.ColorBitboards[pos.ColorToMove], move.FromSquare)

	if move.HasFlag(constants.MoveFlagPromotion) {
		bitboard.SetBit(&pos.PieceBitboards[move.PromotionPiece], move.ToSquare)
	} else {
		bitboard.SetBit(&pos.PieceBitboards[move.Piece], move.ToSquare)
	}
	bitboard.SetBit(&pos.ColorBitboards[pos.ColorToMove], move.ToSquare)

	if move.HasFlag(constants.MoveFlagCapture) {
		if move.HasFlag(constants.MoveFlagEnPassant) {
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

	if move.HasFlag(constants.MoveFlagDoublePawnPush) {
		if isWhite {
			pos.EnPassantSquare = move.ToSquare + 8
		} else {
			pos.EnPassantSquare = move.ToSquare - 8
		}
	} else {
		pos.EnPassantSquare = constants.NoSquare
	}

	if move.HasFlag(constants.MoveFlagCastle) {
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
