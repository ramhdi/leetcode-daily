// https://leetcode.com/problems/smallest-string-starting-from-leaf/
// 2024/04/17

#include <iostream>
#include <string>

#include "TreeNode.hpp"
using namespace std;

class Solution {
   public:
    string smallestFromLeaf(TreeNode* root) { return this->dfs(root, ""); }

   private:
    string dfs(TreeNode* root, string state) {
        if (root == nullptr) {
            return state;
        }

        string newState = (char)('a' + root->val) + state;

        if (root->left == nullptr && root->right == nullptr) {
            // Leaf
            return newState;
        }

        if (root->left == nullptr) {
            return this->dfs(root->right, newState);
        }

        if (root->right == nullptr) {
            return this->dfs(root->left, newState);
        }

        string left = this->dfs(root->left, newState);
        string right = this->dfs(root->right, newState);

        return left < right ? left : right;
    }
};

int main() {
    Solution solution;
    TreeNode* root = createTree({0, 1, 2, 3, 4, 3, 4});
    string result = solution.smallestFromLeaf(root);
    cout << result << endl;  // dba

    root = createTree({25, 1, 3, 1, 3, 0, 2});
    result = solution.smallestFromLeaf(root);
    cout << result << endl;  // adz

    root = createTree({2, 2, 1, -999999, 1, 0, -999999, 0});
    result = solution.smallestFromLeaf(root);
    cout << result << endl;  // abc

    return 0;
}