package main

import (
	"fmt"
	"os"
)

type file_t struct {
	fname string
	size  int64
}

var exclmap = map[string]struct{}{}

func main() {
	args := os.Args[1:]

	exclmap[".git"] = struct{}{}

	subd := make(chan struct{}) // subdirs
	done := make(chan struct{}) // done dirs
	fsz := make(chan file_t)

	if len(args) == 0 {
		cwd, err := os.Getwd()
		if err != nil {
			quit(err)
		}

		go searchDir(cwd, fsz, done, subd)
	}

	subs := 0
	fin := 0
	for {
		select {
		case f := <-fsz:
			fmt.Println(f)
		case <-subd:
			subs++

		case <-done:
			fin++
			if fin >= subs {
				os.Exit(0)
			}
		}
	}

}

func searchDir(dir string, fsz chan<- file_t, done, subd chan struct{}) {
	de, err := os.ReadDir(dir)
	if err != nil {
		e := fmt.Errorf(
			"err parsing %v dir: %w", dir, err,
		)
		panic(e)
	}

	for _, f := range de {
		if f.IsDir() {

			if _, ok := exclmap[f.Name()]; ok {
				continue
			}

			subd <- struct{}{}
			go searchDir(dir+"/"+f.Name(), fsz, done, subd)

		} else {
			info, err := f.Info()
			if err != nil {
				quit(err)
			}

			file := file_t{f.Name(), info.Size()}
			fsz <- file
		}
	}

	done <- struct{}{}
}

func quit(e error) {
	fmt.Println(e)
	os.Exit(1)
}
