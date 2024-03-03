// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

#include <iostream>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
   public:
    ListNode *removeNthFromEnd(ListNode *head, int n) {
        if (head->next == nullptr) {
            return nullptr;
        }

        int k = 0;
        ListNode *curr = head;
        ListNode *toUpdate = head;
        while (curr->next != nullptr) {
            curr = curr->next;
            if (k >= n) {
                toUpdate = toUpdate->next;
            }
            k++;
        }

        if (k == n - 1) {
            return head->next;
        }

        toUpdate->next = toUpdate->next->next;
        return head;
    }

    void printList(ListNode *head) {
        if (head == nullptr) {
            cout << endl;
        } else {
            cout << head->val << ",";
            printList(head->next);
        }
    }
};

int main() {
    Solution solution;
    ListNode *head = new ListNode(
        1, new ListNode(
               2, new ListNode(3, new ListNode(4, new ListNode(5, nullptr)))));
    ListNode *result = solution.removeNthFromEnd(head, 1);

    solution.printList(result);
    return 0;
}