package main

import (
	"fmt"
	"sort"
	"sync"
)

type finfo struct {
	name string
	size int64
}

type results struct {
	files map[string]int64
	mu    sync.Mutex
}

// No-concurrency version of the add func.
func (r *results) addNc(options *opts, root *string, files ...finfo) {
	for _, f := range files {
		if f.size == 0 {
			continue
		}

		if options.recurse || root == nil {
			r.files[f.name] = f.size
			continue
		}

		v, ok := r.files[*root]
		if !ok {
			r.files[*root] = f.size
		} else {
			r.files[*root] = v + f.size
		}
	}
}

func (r *results) sort() []finfo {
	files := make([]finfo, len(r.files))
	idx := 0
	for k, v := range r.files {
		fi := finfo{
			name: k,
			size: v,
		}
		files[idx] = fi
		idx++
	}

	sort.Slice(files, func(i, j int) bool {
		return files[i].size < files[j].size
	})

	return files
}

func (r *results) dump() {
	for k, v := range r.files {
		fmt.Println(k, v)
	}
}
