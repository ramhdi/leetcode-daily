// https://leetcode.com/problems/remove-linked-list-elements/

#include <iostream>

#include "ListNode.hpp"
using namespace std;

class Solution {
   public:
    ListNode* removeElements(ListNode* head, int val) {
        if (head == nullptr) {
            return nullptr;
        }

        if (head->val == val) {
            return removeElements(head->next, val);
        }

        if (head->next == nullptr) {
            return head;
        }

        if (head->next->val == val) {
            head->next = head->next->next;
        }

        head->next = removeElements(head->next, val);
        return head;
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
    ListNode* result = solution.removeElements(head, 6);
    printList(result);
    return 0;
}