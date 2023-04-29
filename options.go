package main

type opts struct {
	// All -> include hidden files / dots.
	all bool

	// Needs to be set when stdout is not a tty,
	// because the color control chars break bins
	// that dux is being piped into.
	color bool

	// Recurse -> print every file + size if true.
	// If not set, then this will summarize dirs.
	recurse bool

	// this is complex enough I want to shunt it
	// into its own flag
	concurrently bool

	args []string
}

func defaultOpts() opts {
	return opts{
		all:          false,
		color:        true,
		recurse:      false,
		concurrently: false,
		args:         []string{},
	}
}
