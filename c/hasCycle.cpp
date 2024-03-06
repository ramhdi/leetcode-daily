// https://leetcode.com/problems/linked-list-cycle/
// 2024/03/06

#include <iostream>
#include <unordered_set>

#include "ListNode.hpp"
using namespace std;

class Solution {
   public:
    bool hasCycle(ListNode *head) {
        std::unordered_set<ListNode *> visited;
        while (head) {
            if (visited.count(head)) {
                return true;
            }
            visited.insert(head);
            head = head->next;
        }
        return false;
    }
};

int main() {
    ListNode *head = new ListNode(3);
    head->next = new ListNode(2);
    head->next->next = new ListNode(0);
    head->next->next->next = new ListNode(-4);
    head->next->next->next->next = head->next;  // Cycle: -4 -> 2

    Solution solution;
    std::cout << (solution.hasCycle(head) ? "Linked list has a cycle"
                                          : "Linked list does not have a cycle")
              << std::endl;

    delete head->next->next->next;
    delete head->next->next;
    delete head->next;
    delete head;

    return 0;
}