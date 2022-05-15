package main

import (
	"fmt"
	"os"
	"sort"
)

type file_t struct {
	fname string
	root  string
	size  int64
}

type size_sl []int64

func (s size_sl) Less(i, j int64) bool {
	return i < j
}

func (s size_sl) Len() int {
	return len(s)
}

const rootStr = "."

func main() {
	args := os.Args[1:]

	subd := make(chan struct{}) // subdirs
	done := make(chan struct{}) // done doing x dirs
	fsz := make(chan file_t)

	if len(args) == 0 {
		cwd, err := os.Getwd()
		if err != nil {
			quit(err)
		}

		go searchDir(cwd, rootStr, fsz, done, subd)
	}

	subs := 0
	fin := 0

	// sof := []file_t{} // short for _slice of files_
	mof := map[string]int64{} // map of files

loop:
	for {
		select {
		case f := <-fsz:
			if _, ok := mof[f.root]; ok {
				mof[f.root] += f.size

			} else {
				mof[f.root] = f.size
			}
			// sof = append(sof, f)

		case <-subd:
			subs++

		case <-done:
			fin++
			if fin >= subs {
				break loop
			}
		}
	}

	//reverse the file map
	fmap := map[int64]string{}
	szs := []int64{}
	for rt, sz := range mof {
		fmap[sz] = rt
		szs = append(szs, sz)
	}

	// for _, f := range sof {
	// 	for _, ok := fmap[f.size]; ok; {
	// 		f.size++
	// 	}

	// 	fmap[f.size] = f.root
	// }

	sort.Slice(szs,
		func(i, j int) bool { return szs[i] < szs[j] },
	)

	// fmt.Printf("%.2fMB\n", sumSzs/1_000_000)

	for _, sz := range szs {
		v := fmap[sz]
		fmt.Println(fmtsz(sz), v)
	}
}

func searchDir(
	dir, pdir string,
	fsz chan<- file_t,
	done, subd chan struct{},
) {
	de, err := os.ReadDir(dir)
	if err != nil {
		e := fmt.Errorf(
			"err parsing %v dir: %w", dir, err,
		)
		panic(e)
	}

	for _, f := range de {

		var parentdir string
		if pdir == rootStr {
			parentdir = rootStr
		} else {
			parentdir = pdir
		}

		if f.IsDir() {
			subd <- struct{}{}
			go searchDir(
				dir+"/"+f.Name(),
				pdir,
				fsz, done, subd,
			)

		} else {
			info, err := f.Info()
			if err != nil {
				quit(err)
			}

			file := file_t{
				fname: f.Name(),
				root:  parentdir,
				size:  info.Size(),
			}

			fsz <- file
		}
	}

	done <- struct{}{}
}

const (
	KB float64 = 1000
	MB         = KB * 1000
	GB         = MB * 1000
)

func fmtsz(sz int64) string {
	s := ""
	sf := float64(sz)
	switch {
	case sf > GB:
		s = fmt.Sprintf("%.2fGB", sf/GB)
	case sf > MB:
		s = fmt.Sprintf("%.2fMB", sf/MB)
	case sf > KB:
		s = fmt.Sprintf("%.2ffKB", sf/KB)
	default:
		s = fmt.Sprintf("%.2f bytes", sf)
	}
	return s
}

func quit(e error) {
	fmt.Println(e)
	os.Exit(1)
}
