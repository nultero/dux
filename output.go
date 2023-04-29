package main

import (
	"fmt"
	"strings"
)

type sizeEnum byte

const (
	KB float64 = 1000.0
	MB float64 = 1000.0 * KB
	GB float64 = 1000.0 * MB
	TB float64 = 1000.0 * GB
	PB float64 = 1000.0 * GB

	offset int = 3
)

// Bytes are just a static gray.
func grayBytes(str string, sz float64) string {
	return fmt.Sprintf("\x1b[38;2;80;80;80m%s\x1b[0m", str)
}

func lukewarmKBytes(str string, sz float64) string {
	colorPercent := sz / MB
	reduxer := 60.0 * colorPercent
	red := 80 + int(reduxer)
	reduxer = 80.0 + (0.65 * reduxer)
	green := int(reduxer)
	blue := int(80.0 - (80.0 * colorPercent))
	return fmt.Sprintf("\x1b[38;2;%d;%d;%dm%s\x1b[0m", red, green, blue, str)
}

func mildMegBytes(str string, sz float64) string {
	colorPercent := sz / GB
	reduxer := 115.0 * colorPercent
	red := 140 + int(reduxer)
	reduxer = 110.5 + (0.65 * reduxer)
	green := int(reduxer)
	return fmt.Sprintf("\x1b[38;2;%d;%d;1m%s\x1b[0m", red, green, str)
}

func hotGigs(str string, sz float64) string {
	colorPercent := sz / TB
	reduxer := 1.0 - colorPercent
	green := int(160.0 * reduxer)
	return fmt.Sprintf("\x1b[38;2;255;%d;1m%s\x1b[0m", green, str)
}

// Just a static, bright red.
func spicyTbytes(str string, sz float64) string {
	return fmt.Sprintf("\x1b[38;2;255;0;0m%s", str)
}

// Just a static, bright red.
func fieryLava(str string, sz float64) string {
	return fmt.Sprintf("\x1b[38;2;255;0;0m%s", str)
}

// Returns the fmt string + apparent length.
func fmtSz(sz float64, color bool) (string, int64) {
	s, chars := "", 0
	var fn func(string, float64) string

	if sz < KB {
		s = fmt.Sprintf("%.0fB", sz)
		chars = len(s)
		fn = grayBytes

	} else if sz < MB {
		s = fmt.Sprintf("%.0fKB", sz/KB)
		chars = len(s)
		fn = lukewarmKBytes

	} else if sz < GB {
		s = fmt.Sprintf("%.0fMB", sz/MB)
		chars = len(s)
		fn = mildMegBytes

	} else if sz < TB {
		s = fmt.Sprintf("%.1fGB", sz/GB)
		chars = len(s)
		fn = hotGigs

	} else if sz < PB {
		s = fmt.Sprintf("%.1fTB", sz/TB)
		chars = len(s)
		fn = spicyTbytes

	} else {
		s = fmt.Sprintf("%.1fPB", sz/PB)
		chars = len(s)
		fn = fieryLava
	}

	if color {
		s = fn(s, sz)
	}

	return s, int64(chars)
}

func printResults(files []finfo, options *opts) {
	var maxlen int64 = 0
	color := options.color
	names := make([]string, len(files))
	for idx, file := range files {
		sz := float64(file.size)
		str, nchars := fmtSz(sz, color)
		if nchars > maxlen {
			maxlen = nchars
		}
		f := finfo{
			name: str,
			size: int64(nchars),
		}
		names[idx] = file.name
		files[idx] = f
	}

	maxlen += int64(offset)

	for idx, file := range files {
		diff := maxlen - file.size
		fmt.Printf(
			" %s%s%s\n",
			file.name,
			strings.Repeat(" ", int(diff)),
			names[idx],
		)
	}
}
