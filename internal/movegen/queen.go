package movegen

type QueenMoveGen struct {
	rookMoveGen   *RookMoveGen
	bishopMoveGen *BishopMoveGen
}

func (qmg *QueenMoveGen) GetAttacks(square uint8, occupancy uint64) uint64 {
	return qmg.rookMoveGen.GetAttacks(square, occupancy) | qmg.bishopMoveGen.GetAttacks(square, occupancy)
}

func NewQueenMoveGen(rmg *RookMoveGen, bmg *BishopMoveGen) *QueenMoveGen {
	return &QueenMoveGen{
		rookMoveGen:   rmg,
		bishopMoveGen: bmg,
	}
}
