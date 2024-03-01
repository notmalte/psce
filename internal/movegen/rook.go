package movegen

import (
	"github.com/notmalte/psce/internal/bitboard"
	"math/bits"
)

type RookMoveGen struct {
	attackCandidateTable [64]uint64
	relevantBitsTable    [64]int
	magicNumbers         [64]uint64
	attackTable          [][]uint64
}

func (rmg *RookMoveGen) generateAttackCandidateTable() [64]uint64 {
	table := [64]uint64{}

	for square := range uint8(64) {
		table[square] = rmg.maskAttackCandidates(square)
	}

	return table
}

func (rmg *RookMoveGen) maskAttackCandidates(square uint8) uint64 {
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

func (rmg *RookMoveGen) generateRelevantBitsTable(attackCandidateTable [64]uint64) [64]int {
	table := [64]int{}

	for square := range uint8(64) {
		table[square] = bits.OnesCount64(attackCandidateTable[square])
	}

	return table
}

func (rmg *RookMoveGen) maskAttacks(square uint8, occupancy uint64) uint64 {
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

func (rmg *RookMoveGen) generateMagicNumbers() [64]uint64 {
	magicNumbers := [64]uint64{}

	for square := range uint8(64) {
		magicNumbers[square] = generateMagicNumber(square, rmg)
	}

	return magicNumbers
}

func (rmg *RookMoveGen) generateAttackTable(candidateTable [64]uint64, magicNumbers [64]uint64) [][]uint64 {
	attackTable := make([][]uint64, 64)

	for square := range uint8(64) {
		candidateMask := candidateTable[square]
		bitsInMask := bits.OnesCount64(candidateMask)
		indexUpperLimit := uint64(1 << bitsInMask)

		attackTable[square] = make([]uint64, maxIndexCount)

		for index := range indexUpperLimit {
			occupancy := maskOccupancy(candidateMask, index)
			magicIndex := calcMagicIndex(occupancy, magicNumbers[square], bitsInMask)
			attackTable[square][magicIndex] = rmg.maskAttacks(square, occupancy)
		}
	}

	return attackTable
}

func (rmg *RookMoveGen) getAttacks(square uint8, occupancy uint64) uint64 {
	maskedOccupancy := occupancy & rmg.attackCandidateTable[square]
	magicNumber := rmg.magicNumbers[square]
	relevantBits := rmg.relevantBitsTable[square]

	magicIndex := calcMagicIndex(maskedOccupancy, magicNumber, relevantBits)

	return rmg.attackTable[square][magicIndex]
}

func newRookMoveGen() *RookMoveGen {
	rmg := &RookMoveGen{}
	rmg.attackCandidateTable = rmg.generateAttackCandidateTable()
	rmg.relevantBitsTable = rmg.generateRelevantBitsTable(rmg.attackCandidateTable)
	rmg.magicNumbers = rmg.generateMagicNumbers()
	rmg.attackTable = rmg.generateAttackTable(rmg.attackCandidateTable, rmg.magicNumbers)

	return rmg
}
