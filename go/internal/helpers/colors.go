package helpers

import "github.com/notmalte/psce/internal/constants"

func ColorString(color uint8) string {
	switch color {
	case constants.ColorWhite:
		return "White"
	case constants.ColorBlack:
		return "Black"
	case constants.ColorBoth:
		return "Both"
	default:
		return "?"
	}
}
