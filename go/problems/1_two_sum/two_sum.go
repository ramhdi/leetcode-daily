package main

import (
	"fmt"
	"reflect"
)

func twoSum(nums []int, target int) []int {
	m := make(map[int]int)
	for i, num := range nums {
		if j, ok := m[target-num]; ok {
			return []int{j, i}
		}
		m[num] = i
	}
	return nil
}

func main() {
	cases := []struct {
		input  []int
		target int
		output []int
	}{
		{[]int{2, 7, 11, 15}, 9, []int{0, 1}},
		{[]int{3, 2, 4}, 6, []int{1, 2}},
		{[]int{3, 3}, 6, []int{0, 1}},
	}

	for i, tc := range cases {
		result := twoSum(tc.input, tc.target)
		pass := reflect.DeepEqual(result, tc.output)
		status := "Pass"
		if !pass {
			status = "Fail"
		}
		fmt.Printf("TestCase %d: Input: %v, Target: %v | Expected: %v, Got: %v | %s\n",
			i+1, tc.input, tc.target, tc.output, result, status)
	}
}
