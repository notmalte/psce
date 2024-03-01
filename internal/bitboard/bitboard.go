package bitboard

import "fmt"

func IndexToRowCol(index uint8) (uint8, uint8) {
	return index / 8, index % 8
}

func IndexToRowColInt8(index uint8) (int8, int8) {
	return int8(index / 8), int8(index % 8)
}

func RowColToIndex(row, col uint8) uint8 {
	return row*8 + col
}

func RowColToIndexInt8(row, col int8) uint8 {
	return uint8(row*8 + col)
}

func GetBit(bb uint64, index uint8) bool {
	return (bb>>index)&1 == 1
}

func SetBit(bb *uint64, index uint8) {
	*bb |= 1 << index
}

func ClearBit(bb *uint64, index uint8) {
	*bb &= ^(1 << index)
}

func GetRowCol(bb uint64, row, col uint8) bool {
	return GetBit(bb, RowColToIndex(row, col))
}

func SetRowCol(bb *uint64, row, col uint8) {
	SetBit(bb, RowColToIndex(row, col))
}

func ClearRowCol(bb *uint64, row, col uint8) {
	ClearBit(bb, RowColToIndex(row, col))
}

func String(bb uint64) string {
	s := ""

	for row := range uint8(8) {
		s += fmt.Sprintf("%d ", 8-row)
		for col := range uint8(8) {
			if GetRowCol(bb, row, col) {
				s += "1 "
			} else {
				s += "0 "
			}
		}
		s += "\n"
	}

	s += "  A B C D E F G H\n"

	return s
}
