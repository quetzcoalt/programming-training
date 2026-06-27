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
        Node* insertNodeAtTail(Node* head, int data) {
            Node *newNode = new Node(data);
            newNode->next = NULL;
            
            Node *current = head;
            
            if (head == NULL) return newNode;
            else {
                while (current->next) current = current->next;
                
                current->next = newNode;
            }
            
            return head;
        }

        void display(Node *head)
        {
            Node *start=head;
            
            while (start) {
                cout << start->data << " ";
                start = start->next; 
            }
        }
};

