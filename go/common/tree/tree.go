package tree

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func NewTreeNode(val int) *TreeNode {
	return &TreeNode{Val: val}
}

// CreateTreeFromSlice creates a binary tree from a slice of integers. `-1` represents nil.
func CreateTreeFromSlice(data []int) *TreeNode {
	if len(data) == 0 {
		return nil
	}
	var root *TreeNode
	nodes := make([]*TreeNode, len(data))
	for i, val := range data {
		if val == -1 {
			nodes[i] = nil
		} else {
			nodes[i] = &TreeNode{Val: val}
		}
		if i == 0 {
			root = nodes[i]
		}
	}
	for i, node := range nodes {
		if node != nil {
			leftIdx := 2*i + 1
			rightIdx := 2*i + 2
			if leftIdx < len(data) {
				node.Left = nodes[leftIdx]
			}
			if rightIdx < len(data) {
				node.Right = nodes[rightIdx]
			}
		}
	}
	return root
}

// CreateSliceFromTree converts a binary tree to a slice of integers. `-1` represents nil.
func CreateSliceFromTree(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	var result []int
	queue := []*TreeNode{root}
	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]
		if node == nil {
			result = append(result, -1)
		} else {
			result = append(result, node.Val)
			queue = append(queue, node.Left, node.Right)
		}
	}
	// Trim trailing -1 values
	for len(result) > 0 && result[len(result)-1] == -1 {
		result = result[:len(result)-1]
	}
	return result
}
