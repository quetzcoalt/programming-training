#include <iostream>
#include <vector>

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
        Node* deleteNode(Node *head, int position) {
            if (head == NULL) return head;

            if (position == 0) return head->next;

            Node *pre = head;
            Node *current = head;

            for (int i = 0; i < position; i++) {
                pre = current;
                current = current->next;
            }

            pre->next = current->next;

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

        void printLinkedList(Node *head)
        {
            Node *start=head;
            
            while (start) {
                cout << start->data << " ";
                start = start->next; 
            }

            cout << endl;
        }
};

int main() {
    Node* head=NULL;
    
    Solution myList;
    
    vector<int> data = {16, 13, 7};

    for (int d : data) {
        head=myList.insertNodeAtTail(head, d);
    }

    head=myList.insertNodeAtPosition(head, 1, 2);

    myList.printLinkedList(head);

    return 0;
}

