package constants

const (
	ColorWhite uint8 = iota
	ColorBlack
	ColorBoth
)

func ColorString(color uint8) string {
	switch color {
	case ColorWhite:
		return "White"
	case ColorBlack:
		return "Black"
	case ColorBoth:
		return "Both"
	default:
		return "?"
	}
}
