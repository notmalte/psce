package bitboard

import "fmt"

type Bitboard uint64

func (b *Bitboard) String() string {
	s := ""

	for rank := 0; rank < 8; rank++ {
		s += fmt.Sprintf("%d ", 8-rank)
		for file := 0; file < 8; file++ {
			if b.GetRowCol(rank, file) {
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

func (b *Bitboard) GetRowCol(row, col int) bool {
	return (*b>>(row*8+col))&1 == 1
}

func (b *Bitboard) SetRowCol(row, col int) {
	*b |= 1 << (row*8 + col)
}

func (b *Bitboard) ClearRowCol(row, col int) {
	*b &= ^(1 << (row*8 + col))
}

func (b *Bitboard) GetIndex(index int) bool {
	return (*b>>index)&1 == 1
}

func (b *Bitboard) SetIndex(index int) {
	*b |= 1 << index
}

func (b *Bitboard) ClearIndex(index int) {
	*b &= ^(1 << index)
}
