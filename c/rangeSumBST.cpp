// https://leetcode.com/problems/range-sum-of-bst/
// 2024/01/08

#include <iostream>

#include "TreeNode.hpp"
using namespace std;

class Solution {
   public:
    int rangeSumBST(TreeNode* root, int low, int high) {
        if (root == nullptr) {
            return 0;
        }
        int result = 0;
        if (root->val >= low && root->val <= high) {
            result = root->val;
        }
        return result + Solution::rangeSumBST(root->left, low, high) +
               Solution::rangeSumBST(root->right, low, high);
    }
};

int main() {
    Solution solution;
    TreeNode* root = new TreeNode(10, new TreeNode(5, new TreeNode(3), new TreeNode(7)), new TreeNode(15, nullptr, new TreeNode(18)));
    printTree(root);
    int result = solution.rangeSumBST(root, 7, 15);
    cout << result << endl;
    return 0;
}