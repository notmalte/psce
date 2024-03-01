package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"math/bits"
)

type BishopMoveGen struct {
	attackCandidateTable [64]uint64
	relevantBitsTable    [64]int
	magicNumbers         [64]uint64
	attackTable          [][]uint64
}

func (bmg *BishopMoveGen) generateAttackCandidateTable() [64]uint64 {
	table := [64]uint64{}

	for square := range uint8(64) {
		table[square] = bmg.maskAttackCandidates(square)
	}

	return table
}

func (bmg *BishopMoveGen) maskAttackCandidates(square uint8) uint64 {
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

func (bmg *BishopMoveGen) generateRelevantBitsTable(attackCandidateTable [64]uint64) [64]int {
	table := [64]int{}

	for square := range uint8(64) {
		table[square] = bits.OnesCount64(attackCandidateTable[square])
	}

	return table
}

func (bmg *BishopMoveGen) maskAttacks(square uint8, occupancy uint64) uint64 {
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

func (bmg *BishopMoveGen) generateMagicNumbers() [64]uint64 {
	magicNumbers := [64]uint64{}

	for square := range uint8(64) {
		magicNumbers[square] = generateMagicNumber(square, bmg)
	}

	return magicNumbers
}

func (bmg *BishopMoveGen) generateAttackTable(candidateTable [64]uint64, magicNumbers [64]uint64) [][]uint64 {
	attackTable := make([][]uint64, 64)

	for square := range uint8(64) {
		candidateMask := candidateTable[square]
		bitsInMask := bits.OnesCount64(candidateMask)
		indexUpperLimit := uint64(1 << bitsInMask)

		attackTable[square] = make([]uint64, maxIndexCount)

		for index := range indexUpperLimit {
			occupancy := maskOccupancy(candidateMask, index)
			magicIndex := calcMagicIndex(occupancy, magicNumbers[square], bitsInMask)
			attackTable[square][magicIndex] = bmg.maskAttacks(square, occupancy)
		}
	}

	return attackTable
}

func (bmg *BishopMoveGen) getAttacks(square uint8, occupancy uint64) uint64 {
	maskedOccupancy := occupancy & bmg.attackCandidateTable[square]
	magicNumber := bmg.magicNumbers[square]
	relevantBits := bmg.relevantBitsTable[square]

	magicIndex := calcMagicIndex(maskedOccupancy, magicNumber, relevantBits)

	return bmg.attackTable[square][magicIndex]
}

func newBishopMoveGen() *BishopMoveGen {
	bmg := &BishopMoveGen{}
	bmg.attackCandidateTable = bmg.generateAttackCandidateTable()
	bmg.relevantBitsTable = bmg.generateRelevantBitsTable(bmg.attackCandidateTable)
	bmg.magicNumbers = bmg.generateMagicNumbers()
	bmg.attackTable = bmg.generateAttackTable(bmg.attackCandidateTable, bmg.magicNumbers)

	return bmg
}
