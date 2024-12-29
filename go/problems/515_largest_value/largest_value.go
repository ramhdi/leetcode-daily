// https://leetcode.com/problems/find-largest-value-in-each-tree-row/
// 2024/12/25

package main

import (
	"fmt"
	. "leetcode_daily/common/tree"
	"reflect"
)

func largestValues(root *TreeNode) []int {
	if root == nil {
		return make([]int, 0)
	}

	result := make([]int, 0)
	queue := []*TreeNode{root}

	for len(queue) > 0 {
		levelSize := len(queue)
		maxVal := queue[0].Val

		for i := 0; i < levelSize; i++ {
			node := queue[0]
			queue = queue[1:]

			if node.Val > maxVal {
				maxVal = node.Val
			}

			if node.Left != nil {
				queue = append(queue, node.Left)
			}

			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}

		result = append(result, maxVal)
	}

	return result
}

func main() {
	cases := []struct {
		root     []int
		expected []int
	}{
		{[]int{1, 3, 2, 5, 3, -1, 9}, []int{1, 3, 9}},
		{[]int{1, 2, 3}, []int{1, 3}},
	}

	for i, tc := range cases {
		result := largestValues(CreateTreeFromSlice(tc.root))
		pass := reflect.DeepEqual(result, tc.expected)
		status := "Pass"
		if !pass {
			status = "Fail"
		}
		fmt.Printf("TestCase %d: root: %v | Expected: %v, Got: %v | %s\n",
			i+1, tc.root, tc.expected, result, status)
	}
}
