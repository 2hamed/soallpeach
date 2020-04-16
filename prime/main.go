package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"runtime"
	"strconv"
	"sync"
)

type Pair struct {
	first  int64
	second int64
}

func main() {
	filePath := os.Args[1]

	f, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	jobs := make(chan Pair)
	drain := make(chan Pair)
	
	wg := &sync.WaitGroup{}
	
	max := runtime.GOMAXPROCS(0)

	for i := 0; i < max; i++ {
		wg.Add(1)
		worker(jobs, drain, wg)
	}

	go func() {
		scanner := bufio.NewScanner(f)
		i := int64(0)
		for scanner.Scan() {
			n, err := strconv.ParseInt(scanner.Text(), 10, 64)
			if err != nil {
				continue
			}
			jobs <- Pair{i, n}
			i++
		}
		close(jobs)
	}()

	lineMap := make(map[int64]int64)
	go func() {
		for {
			p := <-drain
			lineMap[p.first] = p.second
		}
	}()

	wg.Wait()

	for i := int64(0); ; i++ {
		if v, ok := lineMap[i]; ok {
			fmt.Println(v)
			continue
		}
		break
	}
}

func worker(jobs <-chan Pair, drain chan<- Pair, wg *sync.WaitGroup) {
	go func() {
		for {
			p, ok := <-jobs
			if !ok {
				break
			}
			is := isPrime(p.second)
			drain <- Pair{p.first, is}
		}
		wg.Done()
	}()
}

func isPrime(n int64) int64 {
	if n > 2 && n%2 == 0 {
		return 0
	}

	limit := int64(math.Sqrt(float64(n)))
	for i := int64(2); i <= limit; i++ {
		if n%i == 0 {
			return 0
		}
	}
	return 1
}
