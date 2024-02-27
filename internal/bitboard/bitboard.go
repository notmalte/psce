package bitboard

import "fmt"

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

	s += "  a b c d e f g h\n"

	return s
}

func GetRowCol(bb uint64, row, col uint8) bool {
	return (bb>>(row*8+col))&1 == 1
}

func SetRowCol(bb *uint64, row, col uint8) {
	*bb |= 1 << (row*8 + col)
}

func ClearRowCol(bb *uint64, row, col uint8) {
	*bb &= ^(1 << (row*8 + col))
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
