// https://leetcode.com/problems/binary-tree-inorder-traversal/

#include <iostream>
#include <vector>
using namespace std;

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution
{
public:
    vector<int> inorderTraversal(TreeNode *root)
    {
        vector<int> result;
        inorderHelper(root, result);
        return result;
    }

private:
    void inorderHelper(TreeNode *node, vector<int> &result)
    {
        if (node)
        {
            inorderHelper(node->left, result);
            result.push_back(node->val);
            inorderHelper(node->right, result);
        }
    }
};

int main()
{
    Solution solution;
    TreeNode *root = new TreeNode(1, nullptr, new TreeNode(2, new TreeNode(3, nullptr, nullptr), nullptr));
    vector<int> result = solution.inorderTraversal(root);

    for (int n : result)
    {
        cout << n << ",";
    }
    return 0;
}