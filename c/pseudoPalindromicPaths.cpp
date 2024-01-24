// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/
// 2024/01/24

#include <iostream>
#include <unordered_map>

#include "TreeNode.hpp"
using namespace std;

class Solution {
    int result;

   public:
    int pseudoPalindromicPaths(TreeNode* root) {
        result = 0;
        dfs(root, 0);
        root->left = root->right = 0;
        return result;
    }

    bool isPowerOfTwo(int n) { return n == 0 ? true : !(n & (n - 1)); }

    void dfs(TreeNode* root, int value) {
        if (root == nullptr) return;

        value ^= (1 << root->val);

        if (root->left == nullptr && root->right == nullptr) {
            result += static_cast<int>(isPowerOfTwo(value));
            return;
        }

        dfs(root->left, value);
        dfs(root->right, value);
    }
};

int main() {
    Solution solution;
    TreeNode* root =
        new TreeNode(2, new TreeNode(3, new TreeNode(3), new TreeNode(1)),
                     new TreeNode(1, nullptr, new TreeNode(1)));
    printTree(root);
    int result = solution.pseudoPalindromicPaths(root);
    cout << result << endl;
    return 0;
}