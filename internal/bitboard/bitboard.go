package bitboard

import "fmt"

type Bitboard uint64

func (b *Bitboard) String() string {
	s := ""

	for row := range uint8(8) {
		s += fmt.Sprintf("%d ", 8-row)
		for col := range uint8(8) {
			if b.GetRowCol(row, col) {
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

func (b *Bitboard) GetRowCol(row, col uint8) bool {
	return (*b>>(row*8+col))&1 == 1
}

func (b *Bitboard) SetRowCol(row, col uint8) {
	*b |= 1 << (row*8 + col)
}

func (b *Bitboard) ClearRowCol(row, col uint8) {
	*b &= ^(1 << (row*8 + col))
}

func (b *Bitboard) GetBit(index uint8) bool {
	return (*b>>index)&1 == 1
}

func (b *Bitboard) SetBit(index uint8) {
	*b |= 1 << index
}

func (b *Bitboard) ClearBit(index uint8) {
	*b &= ^(1 << index)
}
