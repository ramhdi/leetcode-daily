// https://leetcode.com/problems/add-one-row-to-tree/
// 2024/04/16

#include <iostream>
#include <queue>

#include "TreeNode.hpp"
using namespace std;

class Solution {
   public:
    TreeNode* addOneRow(TreeNode* root, int val, int depth) {
        if (depth == 1) {
            TreeNode* newRoot = new TreeNode(val);
            newRoot->left = root;
            return newRoot;
        }

        queue<TreeNode*> queue;
        queue.push(root);
        int currentDepth = 1;

        while (!queue.empty() && currentDepth < depth - 1) {
            int levelSize = queue.size();
            for (int i = 0; i < levelSize; ++i) {
                TreeNode* node = queue.front();
                queue.pop();
                if (node->left) queue.push(node->left);
                if (node->right) queue.push(node->right);
            }
            currentDepth++;
        }

        while (!queue.empty()) {
            TreeNode* current = queue.front();
            queue.pop();

            TreeNode* tempLeft = current->left;
            TreeNode* tempRight = current->right;

            current->left = new TreeNode(val);
            current->left->left = tempLeft;

            current->right = new TreeNode(val);
            current->right->right = tempRight;
        }

        return root;
    }
};

int main() {
    Solution solution;
    TreeNode* root = createTree({4, 2, 6, 3, 1, 5});
    TreeNode* result = solution.addOneRow(root, 1, 2);
    printTree(result);  // [4,1,1,2,null,null,6,3,1,5]

    root = createTree({4, 2, -999999, 3, 1});
    result = solution.addOneRow(root, 1, 3);
    printTree(result);  // [4,2,null,1,1,3,null,null,1]
    return 0;
}