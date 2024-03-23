// https://leetcode.com/problems/reorder-list/
// 2024/03/23

#include <iostream>

#include "ListNode.hpp"
using namespace std;

class Solution {
   public:
    void reorderList(ListNode* head) {
        if (head == nullptr) return;

        // Split list into two
        ListNode* curr = head;
        ListNode* secondHalf = head;
        while (curr->next != nullptr && curr->next->next != nullptr) {
            curr = curr->next->next;
            secondHalf = secondHalf->next;
        }
        if (curr->next != nullptr) {
            secondHalf = secondHalf->next;
        }

        // Reverse the second half
        secondHalf = reverseList(secondHalf);

        // Merge list
        while (head != nullptr && secondHalf != nullptr) {
            ListNode* next = head->next;
            ListNode* nextSecondHalf = secondHalf->next;
            head->next = secondHalf;
            secondHalf->next = next;
            head = next;
            secondHalf = nextSecondHalf;
        }

        // Prevent circular loop
        if (head != nullptr && head->next != nullptr) {
            head->next->next = nullptr;
        }
    }

    inline ListNode* reverseList(ListNode* head) {
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
    vector<int> list;
    ListNode* head;

    list = {1, 2, 3, 4};
    head = createLinkedList(list);
    solution.reorderList(head);
    printLinkedList(head);  // [1,4,2,3]

    list = {1, 2, 3, 4, 5};
    head = createLinkedList(list);
    solution.reorderList(head);
    printLinkedList(head);  // [1,5,2,4,3]

    return 0;
}