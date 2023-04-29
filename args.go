package main

import (
	"os"
)

func parseArgs() opts {
	args := os.Args[1:]
	options := defaultOpts()

	doCwd := true

	if len(args) != 0 {
		for _, arg := range args {
			if arg[0] == '-' && len(arg) > 1 {
				for _, c := range arg[1:] {
					switch c {
					case 'a':
						options.all = true
					case 'r':
						options.recurse = true
					case 'c':
						options.concurrently = true
					case 'n':
						options.color = false
					}
				}
				// TODO
				continue
			} else {
				doCwd = false
			}
			options.args = append(options.args, arg)
		}
	}

	if doCwd {
		cwd, _ := os.ReadDir(".")
		for _, entry := range cwd {
			name := entry.Name()
			if name[0] == '.' && !options.all {
				continue
			}
			options.args = append(options.args, name)
		}

	}

	tty := isatty(os.Stdout.Fd())
	if !tty {
		options.color = false
	}
	return options
}
