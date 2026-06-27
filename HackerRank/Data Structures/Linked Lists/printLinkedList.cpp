#include <iostream>

using namespace std;

class Node {
    public:
        int data;
        Node *next;
        Node (int d) {
            data=d;
            next=NULL;
        }
};

class Solution {
    public:
        void printLinkedList(Node *head)
        {
            Node *start=head;
            
            while (start) {
                cout << start->data << " ";
                start = start->next; 
            }
        }
};
