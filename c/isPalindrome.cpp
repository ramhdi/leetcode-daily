// https://leetcode.com/problems/palindrome-linked-list/

#include <iostream>

#include "ListNode.hpp"
using namespace std;

class Solution {
   public:
    bool isPalindrome(ListNode *head) {
        ListNode *slow = head;
        ListNode *fast = head;
        ListNode *prev, *temp;

        while (fast != nullptr && fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
        }

        prev = slow;
        slow = slow->next;
        prev->next = nullptr;
        while (slow != nullptr) {
            temp = slow->next;
            slow->next = prev;
            prev = slow;
            slow = temp;
        }

        fast = head;
        slow = prev;
        while (slow != nullptr) {
            if (fast->val != slow->val) {
                return false;
            } else {
                fast = fast->next;
                slow = slow->next;
            }
        }

        return true;
    }
};

int main() {
    Solution solution;
    ListNode *head = new ListNode(
        1, new ListNode(
               4, new ListNode(5, new ListNode(4, new ListNode(1, nullptr)))));
    printList(head);
    bool result = solution.isPalindrome(head);
    cout << result;
    return 0;
}