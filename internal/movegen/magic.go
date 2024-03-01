package movegen

import (
	"math/bits"
	"math/rand/v2"
)

const maxIndexCount = 1 << 12
const maxMagicNumberGenerationAttempts = 1_000_000_000

type candidateMoveGen interface {
	maskAttackCandidates(square uint8) uint64
	maskAttacks(square uint8, occupancy uint64) uint64
}

func generateMagicNumberCandidate() uint64 {
	a := rand.Uint64()
	b := rand.Uint64()
	c := rand.Uint64()

	return a & b & c
}

func calcMagicIndex(occupancy uint64, magicNumber uint64, bitsInMask int) uint64 {
	return (occupancy * magicNumber) >> (64 - bitsInMask)
}

func generateMagicNumber(square uint8, cmg candidateMoveGen) uint64 {
	occupancies := [maxIndexCount]uint64{}
	attacks := [maxIndexCount]uint64{}

	candidateMask := cmg.maskAttackCandidates(square)
	bitsInMask := bits.OnesCount64(candidateMask)
	indexUpperLimit := uint64(1 << bitsInMask)

	for index := uint64(0); index < indexUpperLimit; index++ {
		occupancies[index] = maskOccupancy(candidateMask, index)
		attacks[index] = cmg.maskAttacks(square, occupancies[index])
	}

outer:
	for range maxMagicNumberGenerationAttempts {
		magicNumberCandidate := generateMagicNumberCandidate()

		if bits.OnesCount64((candidateMask*magicNumberCandidate)&0xFF00_0000_0000_0000) < 6 {
			continue
		}

		usedAttacks := [maxIndexCount]uint64{}

		for index := range indexUpperLimit {
			magicIndex := calcMagicIndex(occupancies[index], magicNumberCandidate, bitsInMask)

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
