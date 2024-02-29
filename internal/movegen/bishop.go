package movegen

import "github.com/notmalte/psce/internal/bitboard"

type BishopMoveGen struct{}

func (bmg *BishopMoveGen) GenerateAttackCandidateTable() [64]uint64 {
	table := [64]uint64{}

	for square := range uint8(64) {
		table[square] = bmg.MaskAttackCandidates(square)
	}

	return table
}

func (bmg *BishopMoveGen) MaskAttackCandidates(square uint8) uint64 {
	attacks := uint64(0)

	squareRow, squareCol := bitboard.IndexToRowColInt8(square)

	row, col := squareRow+1, squareCol+1
	for row < 7 && col < 7 {
		attacks |= 1 << bitboard.RowColToIndexInt8(row, col)
		row++
		col++
	}

	row, col = squareRow-1, squareCol-1
	for row > 0 && col > 0 {
		attacks |= 1 << bitboard.RowColToIndexInt8(row, col)
		row--
		col--
	}

	row, col = squareRow-1, squareCol+1
	for row > 0 && col < 7 {
		attacks |= 1 << bitboard.RowColToIndexInt8(row, col)
		row--
		col++
	}

	row, col = squareRow+1, squareCol-1
	for row < 7 && col > 0 {
		attacks |= 1 << bitboard.RowColToIndexInt8(row, col)
		row++
		col--
	}

	return attacks
}

func (bmg *BishopMoveGen) MaskAttacks(square uint8, occupancy uint64) uint64 {
	attacks := uint64(0)

	squareRow, squareCol := bitboard.IndexToRowColInt8(square)

	row, col := squareRow+1, squareCol+1
	for row <= 7 && col <= 7 {
		mask := uint64(1) << bitboard.RowColToIndexInt8(row, col)
		attacks |= mask

		if mask&occupancy != 0 {
			break
		}

		row++
		col++
	}

	row, col = squareRow-1, squareCol-1
	for row >= 0 && col >= 0 {
		mask := uint64(1) << bitboard.RowColToIndexInt8(row, col)
		attacks |= mask

		if mask&occupancy != 0 {
			break
		}

		row--
		col--
	}

	row, col = squareRow-1, squareCol+1
	for row >= 0 && col <= 7 {
		mask := uint64(1) << bitboard.RowColToIndexInt8(row, col)
		attacks |= mask

		if mask&occupancy != 0 {
			break
		}

		row--
		col++
	}

	row, col = squareRow+1, squareCol-1
	for row <= 7 && col >= 0 {
		mask := uint64(1) << bitboard.RowColToIndexInt8(row, col)
		attacks |= mask

		if mask&occupancy != 0 {
			break
		}

		row++
		col--
	}

	return attacks
}

func (bmg *BishopMoveGen) GenerateMagicNumbers() [64]uint64 {
	magicNumbers := [64]uint64{}

	for square := range uint8(64) {
		magicNumbers[square] = GenerateMagicNumber(square, bmg)
	}

	return magicNumbers
}
