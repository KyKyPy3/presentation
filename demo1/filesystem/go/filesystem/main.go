package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func main() {
	// Open the file
	file, err := os.Open("hello.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		return
	}
	defer file.Close()

	// Read the file contents
	content, err := ioutil.ReadAll(file)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}

	// Print the file contents
	fmt.Printf("%s", content)
}
