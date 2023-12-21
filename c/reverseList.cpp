// https://leetcode.com/problems/reverse-linked-list/

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
};

int main() {
    Solution solution;
    ListNode* head = new ListNode(
        1, new ListNode(
               2, new ListNode(3, new ListNode(4, new ListNode(5, nullptr)))));
    printList(head);
    ListNode* result = solution.reverseList(head);
    printList(result);
    return 0;
}