package movegen

import (
	"math/bits"
	"math/rand/v2"
)

const MaxIndexCount = 1 << 12
const MaxMagicNumberGenerationAttempts = 1_000_000_000

type CandidateMoveGen interface {
	MaskAttackCandidates(square uint8) uint64
	MaskAttacks(square uint8, occupancy uint64) uint64
}

func GenerateMagicNumberCandidate() uint64 {
	a := rand.Uint64()
	b := rand.Uint64()
	c := rand.Uint64()

	return a & b & c
}

func GenerateMagicNumber(square uint8, cmg CandidateMoveGen) uint64 {
	occupancies := [MaxIndexCount]uint64{}
	attacks := [MaxIndexCount]uint64{}

	candidateMask := cmg.MaskAttackCandidates(square)
	bitsInMask := bits.OnesCount64(candidateMask)
	indexUpperLimit := uint64(1 << bitsInMask)

	for index := uint64(0); index < indexUpperLimit; index++ {
		occupancies[index] = MaskOccupancy(candidateMask, index)
		attacks[index] = cmg.MaskAttacks(square, occupancies[index])
	}

outer:
	for range MaxMagicNumberGenerationAttempts {
		magicNumberCandidate := GenerateMagicNumberCandidate()

		if bits.OnesCount64((candidateMask*magicNumberCandidate)&0xFF00_0000_0000_0000) < 6 {
			continue
		}

		usedAttacks := [MaxIndexCount]uint64{}

		for index := uint64(0); index < indexUpperLimit; index++ {
			magicIndex := (occupancies[index] * magicNumberCandidate) >> (64 - bitsInMask)

			if usedAttacks[magicIndex] == 0 {
				usedAttacks[magicIndex] = attacks[index]
			} else if usedAttacks[magicIndex] != attacks[index] {
				continue outer
			}
		}

		return magicNumberCandidate
	}

	panic("Failed to generate magic number")
}
