package movegen

import "github.com/notmalte/psce/internal/bitboard"

type RookMoveGen struct{}

func (rmg *RookMoveGen) GenerateAttackCandidateTable() [64]uint64 {
	table := [64]uint64{}

	for square := range uint8(64) {
		table[square] = rmg.MaskAttackCandidates(square)
	}

	return table
}

func (rmg *RookMoveGen) MaskAttackCandidates(square uint8) uint64 {
	attacks := uint64(0)

	squareRow, squareCol := bitboard.IndexToRowColInt8(square)

	for row := squareRow + 1; row < 7; row++ {
		attacks |= 1 << bitboard.RowColToIndexInt8(row, squareCol)
	}

	for row := squareRow - 1; row > 0; row-- {
		attacks |= 1 << bitboard.RowColToIndexInt8(row, squareCol)
	}

	for col := squareCol + 1; col < 7; col++ {
		attacks |= 1 << bitboard.RowColToIndexInt8(squareRow, col)
	}

	for col := squareCol - 1; col > 0; col-- {
		attacks |= 1 << bitboard.RowColToIndexInt8(squareRow, col)
	}

	return attacks
}

func (rmg *RookMoveGen) MaskAttacks(square uint8, occupancy uint64) uint64 {
	attacks := uint64(0)

	squareRow, squareCol := bitboard.IndexToRowColInt8(square)

	for row := squareRow + 1; row <= 7; row++ {
		mask := uint64(1) << bitboard.RowColToIndexInt8(row, squareCol)
		attacks |= mask
		if occupancy&mask != 0 {
			break
		}
	}

	for row := squareRow - 1; row >= 0; row-- {
		mask := uint64(1) << bitboard.RowColToIndexInt8(row, squareCol)
		attacks |= mask
		if occupancy&mask != 0 {
			break
		}
	}

	for col := squareCol + 1; col <= 7; col++ {
		mask := uint64(1) << bitboard.RowColToIndexInt8(squareRow, col)
		attacks |= mask
		if occupancy&mask != 0 {
			break
		}
	}

	for col := squareCol - 1; col >= 0; col-- {
		mask := uint64(1) << bitboard.RowColToIndexInt8(squareRow, col)
		attacks |= mask
		if occupancy&mask != 0 {
			break
		}
	}

	return attacks
}
