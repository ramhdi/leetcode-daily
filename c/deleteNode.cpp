// https://leetcode.com/problems/delete-node-in-a-linked-list/

#include <iostream>

#include "ListNode.hpp"
using namespace std;

class Solution {
   public:
    void deleteNode(ListNode* node) {
        if (node == nullptr || node->next == nullptr) {
            return;
        }

        ListNode* nextNode = node->next;
        node->val = nextNode->val;
        node->next = nextNode->next;
        delete nextNode;
    }
};

int main() {
    Solution solution;
    ListNode* head = new ListNode(
        1,
        new ListNode(
            2,
            new ListNode(
                6, new ListNode(
                       3, new ListNode(4, new ListNode(5, new ListNode(6)))))));
    printList(head);
    solution.deleteNode(head->next->next);
    printList(head);
    return 0;
}