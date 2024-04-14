// https://leetcode.com/problems/sum-of-left-leaves/
// 2024/04/14

#include <iostream>

#include "TreeNode.hpp"
using namespace std;

class Solution {
   public:
    int sumOfLeftLeaves(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }

        int result = 0;
        if (root->left != nullptr && root->left->left == nullptr &&
            root->left->right == nullptr) {
            result += root->left->val;
        }

        result += this->sumOfLeftLeaves(root->left);
        result += this->sumOfLeftLeaves(root->right);

        return result;
    }
};

int main() {
    Solution solution;
    TreeNode* root =
        new TreeNode(3, new TreeNode(9),
                     new TreeNode(20, new TreeNode(15), new TreeNode(7)));

    int result = solution.sumOfLeftLeaves(root);
    cout << result << endl;  // 24
    return 0;
}