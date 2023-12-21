// https://leetcode.com/problems/swap-nodes-in-pairs/

#include <iostream>

#include "ListNode.hpp"
using namespace std;

class Solution {
   public:
    ListNode* swapPairs(ListNode* head) {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }

        int i = 0;
        ListNode* toSwap = head;
        ListNode* before = head;
        while (toSwap->next != nullptr) {
            if (i % 2 == 0) {
                ListNode* toSwap2 = toSwap->next;
                toSwap->next = toSwap->next->next;
                toSwap2->next = toSwap;

                if (i == 0) {
                    before = toSwap2;
                    head = before;
                } else {
                    before->next = toSwap2;
                }
            }
            // printList(head);

            if (i != 0) {
                before = before->next;
            }
            toSwap = before->next;
            i++;
        }
        return head;
    }
};

int main() {
    Solution solution;
    ListNode* head = new ListNode(
        1, new ListNode(
               2, new ListNode(3, new ListNode(4, new ListNode(5, nullptr)))));
    printList(head);
    ListNode* result = solution.swapPairs(head);
    printList(result);
    return 0;
}