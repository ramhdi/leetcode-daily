// https://leetcode.com/problems/palindrome-linked-list/

#include <iostream>
#include <unordered_set>

#include "ListNode.hpp"
using namespace std;

class Solution {
   public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        unordered_set<ListNode *> nodesA, nodesB;
        ListNode *currA = headA;
        ListNode *currB = headB;
        while (currA != nullptr || currB != nullptr) {
            if (nodesA.find(currB) != nodesA.end()) {
                return currB;
            }
            if (currB != nullptr) {
                nodesB.insert(currB);
                currB = currB->next;
            }
            if (nodesB.find(currA) != nodesB.end()) {
                return currA;
            }
            if (currA != nullptr) {
                nodesA.insert(currA);
                currA = currA->next;
            }
        }
        return nullptr;
    }
};

int main() {
    Solution solution;
    ListNode *headA = new ListNode(
        4, new ListNode(
               1, new ListNode(8, new ListNode(4, new ListNode(5, nullptr)))));
    ListNode *headB =
        new ListNode(5, new ListNode(6, new ListNode(1, headA->next->next)));
    printList(headA);
    printList(headB);
    ListNode *result = solution.getIntersectionNode(headA, headB);
    printList(result);
    return 0;
}