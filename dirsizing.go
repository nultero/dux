package main

import (
	"log"
	"os"
	"path/filepath"
)

// Single-threaded version of dirsizing.
func stGetSize(file string, root *string, r *results, options *opts) {
	stat, err := os.Stat(file)
	if err != nil {
		log.Printf("err stat'ing: %s, %v\n", file, err)
		return
	}

	if stat.IsDir() {
		entries, err := os.ReadDir(file)
		if err != nil {
			log.Printf("err reading dir: %s, %v\n", file, err)
		}

		for _, ent := range entries {
			path := filepath.Join(file, ent.Name())
			var ptr *string
			if root == nil {
				ptr = &file
			} else {
				ptr = root
			}
			stGetSize(path, ptr, r, options)
		}
		return
	}

	f := finfo{
		name: file,
		size: stat.Size(),
	}
	r.addNc(options, root, f)
}
