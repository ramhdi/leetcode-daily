// https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/
// 2024/01/10

#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "TreeNode.hpp"
using namespace std;

void printUG(const unordered_map<int, vector<int>>& ugTree) {
    for (const auto& entry : ugTree) {
        std::cout << "Key: " << entry.first << ", Values: [";

        const auto& values = entry.second;
        for (size_t i = 0; i < values.size(); ++i) {
            std::cout << values[i];
            if (i < values.size() - 1) {
                std::cout << ", ";
            }
        }

        std::cout << "]" << std::endl;
    }
}

void printVector(const vector<int>& vec) {
    std::cout << "[";

    for (size_t i = 0; i < vec.size(); ++i) {
        std::cout << vec[i];
        if (i < vec.size() - 1) {
            std::cout << ", ";
        }
    }

    std::cout << "]" << std::endl;
}

class Solution {
   public:
    int amountOfTime(TreeNode* root, int start) {
        unordered_map<int, vector<int>> ugTree;
        unordered_set<int> visitedNodes;
        Solution::dfs(ugTree, root);

        printUG(ugTree);

        vector<int> startNodes{start};
        int result = 0;
        while (startNodes.size() != 0) {
            printVector(startNodes);
            vector<int> nextNodes;
            for (int node : startNodes) {
                visitedNodes.insert(node);
                for (int next : ugTree[node]) {
                    if (visitedNodes.find(next) == visitedNodes.end()) {
                        nextNodes.push_back(next);
                    }
                }
            }
            if (nextNodes.size() != 0) {
                result++;
            }
            startNodes = nextNodes;
        }
        return result;
    }

    void dfs(unordered_map<int, vector<int>>& ugTree, TreeNode* root) {
        if (root == nullptr) {
            return;
        }

        if (root->left != nullptr) {
            ugTree[root->val].push_back(root->left->val);
            ugTree[root->left->val].push_back(root->val);
        }

        if (root->right != nullptr) {
            ugTree[root->val].push_back(root->right->val);
            ugTree[root->right->val].push_back(root->val);
        }

        Solution::dfs(ugTree, root->left);
        Solution::dfs(ugTree, root->right);
    }
};

int main() {
    Solution solution;
    TreeNode* root = new TreeNode(
        1,
        new TreeNode(5, nullptr,
                     new TreeNode(4, new TreeNode(9), new TreeNode(2))),
        new TreeNode(3, new TreeNode(10), new TreeNode(6)));
    printTree(root);
    int result = solution.amountOfTime(root, 3);
    cout << result << endl;
    return 0;
}