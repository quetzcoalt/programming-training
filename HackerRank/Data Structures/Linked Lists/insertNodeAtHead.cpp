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
        Node* insertNodeAtHead(Node* head, int data) {
            Node *temp = new Node(data);
            temp->next = NULL;

            if (head == NULL) return temp;
            else {
                temp->next = head;
                head = temp;
            }

            return head;
        }

        Node* insertNodeAtPosition(Node* head, int data, int position) {
            Node *newNode = new Node(data);
            newNode->next = NULL;

            if (head == NULL) return newNode;

            Node *current, *pre = head;

            for (int i = 0; i < position; i++) {
                pre = current;
                current = current->next;
            }

            pre->next = newNode;
            newNode->next = current;

            return head;
        }

        void printLinkedList(Node *head)
        {
            Node *start=head;
            
            while (start) {
                cout << start->data << " ";
                start = start->next; 
            }
        }
};

int main() {
    Node* head=NULL;
    
    Solution myList;
    
    int T, data;
    cin >> T;

    while (T-->0) {
        cin >> data;
        head=myList.insertNodeAtHead(head, data);
    }

    myList.printLinkedList(head);

    return 0;
}

