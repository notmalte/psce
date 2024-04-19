package tt

const (
	FlagExact = uint(iota)
	FlagAlpha
	FlagBeta
)

type Entry struct {
	Hash  uint64
	Depth uint
	Flag  uint
	Score int
}

type TranspositionTable struct {
	entries []Entry
	size    uint64
}

func NewTranspositionTable(size uint64) *TranspositionTable {
	if size == 0 || size&(size-1) != 0 {
		panic("size must be a power of 2")
	}

	entries := make([]Entry, size)

	return &TranspositionTable{
		entries: entries,
		size:    size,
	}
}

func (tt *TranspositionTable) Size() uint64 {
	return tt.size
}

func (tt *TranspositionTable) Clear() {
	for i := range tt.entries {
		tt.entries[i] = Entry{}
	}
}

func (tt *TranspositionTable) Store(hash uint64, depth uint, flag uint, score int) {
	entry := &tt.entries[hash%tt.size]
	entry.Hash = hash
	entry.Depth = depth
	entry.Flag = flag
	entry.Score = score
}

func (tt *TranspositionTable) Probe(hash uint64, depth uint, alpha int, beta int) (int, bool) {
	entry := tt.entries[hash%tt.size]

	if entry.Hash == hash && entry.Depth >= depth {
		switch entry.Flag {
		case FlagExact:
			return entry.Score, true
		case FlagAlpha:
			if entry.Score <= alpha {
				return alpha, true
			}
		case FlagBeta:
			if entry.Score >= beta {
				return beta, true
			}
		}
	}

	return 0, false
}
