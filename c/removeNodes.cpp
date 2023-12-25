// https://leetcode.com/problems/remove-nodes-from-linked-list/

#include <iostream>

#include "ListNode.hpp"
using namespace std;

class Solution {
   public:
    ListNode* reverseList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }

        ListNode* next = reverseList(head->next);
        head->next->next = head;
        head->next = nullptr;
        return next;
    }

    ListNode* removeNodes(ListNode* head) {
        ListNode* tail = reverseList(head);
        printList(tail);
        ListNode* curr = tail;
        int maxVal = 0;
        while (curr->next->next != nullptr) {
            maxVal = max(maxVal, curr->val);
            if (curr->next->val < maxVal) {
                curr->next = curr->next->next;
            }
            if (curr->next->val >= maxVal && curr->next->next != nullptr) {
                curr = curr->next;
            }
        }
        if (curr->next->val < maxVal) {
            curr->next = nullptr;
        }
        return reverseList(tail);
    }
};

int main() {
    Solution solution;
    ListNode* head = new ListNode(
        5, new ListNode(
               2, new ListNode(13, new ListNode(3, new ListNode(8, nullptr)))));
    printList(head);
    ListNode* result = solution.removeNodes(head);
    printList(result);
    return 0;
}