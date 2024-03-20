// https://leetcode.com/problems/merge-in-between-linked-lists/
// 2024/03/20

#include <iostream>

#include "ListNode.hpp"
using namespace std;

class Solution {
   public:
    ListNode* mergeInBetween(ListNode* list1, int a, int b, ListNode* list2) {
        ListNode *before_a = nullptr, *after_b = nullptr;
        ListNode* current = list1;

        int i = 0;
        while (current != nullptr) {
            if (i == b) {
                after_b = current->next;
                current->next = nullptr;
            }
            current = current->next;
            ++i;
        }

        current = list2;
        while (current->next != nullptr) {
            current = current->next;
        }
        current->next = after_b;

        current = list1;
        i = 0;
        while (current->next != nullptr && i != a - 1) {
            current = current->next;
            ++i;
        }
        current->next = list2;

        return list1;
    }
};

int main() {
    Solution solution;
    vector<int> list1, list2;
    list1 = {10, 1, 13, 6, 9, 5};
    list2 = {1000000, 1000001, 1000002};
    printLinkedList(solution.mergeInBetween(
        createLinkedList(list1), 3, 4,
        createLinkedList(list2)));  // [10,1,13,1000000,1000001,1000002,5]

    list1 = {0, 1, 2, 3, 4, 5, 6};
    list2 = {1000000, 1000001, 1000002, 1000003, 1000004};
    printLinkedList(solution.mergeInBetween(
        createLinkedList(list1), 2, 5,
        createLinkedList(
            list2)));  // [0,1,1000000,1000001,1000002,1000003,1000004,6]
    return 0;
}