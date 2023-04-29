package main

import (
	"fmt"
	"sync"
)

/*
	Not going to care too much about block size:
	My version of du is about large-scope stuff,
	like the *most* fat files / dirs
*/

func main() {
	options := parseArgs()
	if options.concurrently {
		fmt.Println("not done yet")
		return
	}

	res := results{
		files: map[string]int64{},
		mu:    sync.Mutex{},
	}

	for _, arg := range options.args {
		stGetSize(arg, nil, &res, &options)
	}

	printResults(res.sort(), &options)
}
