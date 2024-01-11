// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
// 2024/01/11

#include <iostream>

#include "TreeNode.hpp"
using namespace std;

/*
struct DfsResult {
    int self;
    int minNode;
    int maxNode;
    int maxDiff;
};

class Solution {
   public:
    int maxAncestorDiff(TreeNode* root) { return dfs(root).maxDiff; }

    DfsResult dfs(TreeNode* root) {
        if (root == nullptr) {
            return DfsResult{0, 0, 0, 0};
        }

        if (root->left == nullptr && root->right == nullptr) {
            return DfsResult{root->val, root->val, root->val, 0};
        }

        DfsResult result{root->val, INT_MAX, INT_MIN, 0};
        DfsResult result1{0, INT_MAX, INT_MIN, 0};
        DfsResult result2{0, INT_MAX, INT_MIN, 0};

        if (root->left != nullptr) {
            result1 = dfs(root->left);
        }

        if (root->right != nullptr) {
            result2 = dfs(root->right);
        }

        result.minNode = min({root->val, result1.minNode, result2.minNode});
        result.maxNode = max({root->val, result1.maxNode, result2.maxNode});
        result.maxDiff =
            max({result1.maxDiff, result2.maxDiff, (root->val - result.minNode),
                 abs(root->val - result.maxNode)});
        return result;
    }
};
*/

class Solution {
   private:
    int diff = 0;

    void findDiff(TreeNode* root, int minVal, int maxVal) {
        if (!root) return;
        diff = max(diff, max(abs(minVal - root->val), abs(maxVal - root->val)));
        minVal = min(minVal, root->val);
        maxVal = max(maxVal, root->val);
        findDiff(root->left, minVal, maxVal);
        findDiff(root->right, minVal, maxVal);
    }

   public:
    int maxAncestorDiff(TreeNode* root) {
        if (!root) return 0;
        int minVal = root->val, maxVal = root->val;
        findDiff(root, minVal, maxVal);
        return diff;
    }
};

int main() {
    Solution solution;
    TreeNode* root = new TreeNode(
        8,
        new TreeNode(3, new TreeNode(1),
                     new TreeNode(6, new TreeNode(4), new TreeNode(7))),
        new TreeNode(10, nullptr, new TreeNode(14, new TreeNode(13), nullptr)));
    TreeNode* root2 =
        new TreeNode(2, nullptr, new TreeNode(0, new TreeNode(1), nullptr));
    printTree(root);
    int result = solution.maxAncestorDiff(root);
    cout << endl << result << endl;
    return 0;
}