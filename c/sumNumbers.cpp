// https://leetcode.com/problems/sum-root-to-leaf-numbers/
// 2024/04/15

#include <iostream>

#include "TreeNode.hpp"
using namespace std;

class Solution {
   public:
    int sumNumbers(TreeNode* root) { return this->dfs(root, 0); }

   private:
    int dfs(TreeNode* root, int state) {
        if (root == nullptr) {
            return 0;
        }

        if (root->left == nullptr && root->right == nullptr) {
            // cout << state * 10 + root->val << endl;
            return state * 10 + root->val;
        }

        int result = 0;
        if (root->left != nullptr) {
            result += this->dfs(root->left, 10 * state + root->val);
        }

        if (root->right != nullptr) {
            result += this->dfs(root->right, 10 * state + root->val);
        }

        return result;
    }
};

int main() {
    Solution solution;
    TreeNode* root = new TreeNode(1, new TreeNode(2), new TreeNode(3));

    int result = solution.sumNumbers(root);
    cout << result << endl;  // 25

    root = new TreeNode(4, new TreeNode(9, new TreeNode(5), new TreeNode(1)),
                        new TreeNode(0));

    result = solution.sumNumbers(root);
    cout << result << endl;  // 1026
    return 0;
}