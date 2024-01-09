// https://leetcode.com/problems/leaf-similar-trees/
// 2024/01/09

#include <iostream>
#include <vector>

#include "TreeNode.hpp"
using namespace std;

class Solution {
   public:
    bool leafSimilar(TreeNode* root1, TreeNode* root2) {
        vector<int> leaf1, leaf2;
        Solution::dfs(root1, leaf1);
        Solution::dfs(root2, leaf2);

        return leaf1 == leaf2;
    }

    void dfs(TreeNode* root, vector<int>& leaf) {
        if (root == nullptr) {
            return;
        }
        if (root->left == nullptr && root->right == nullptr) {
            leaf.push_back(root->val);
            return;
        }
        dfs(root->left, leaf);
        dfs(root->right, leaf);
    }
};

int main() {
    Solution solution;
    TreeNode* root = new TreeNode(
        3,
        new TreeNode(5, new TreeNode(6),
                     new TreeNode(2, new TreeNode(7), new TreeNode(4))),
        new TreeNode(1, new TreeNode(9), new TreeNode(8)));
    printTree(root);
    bool result = solution.leafSimilar(root, root);
    cout << result << endl;
    return 0;
}