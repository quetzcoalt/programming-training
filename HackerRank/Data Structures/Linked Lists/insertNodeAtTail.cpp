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

int main() {
    Node* head=NULL;
    
    Solution myList;
    
    int T, data;
    cin >> T;

    while (T-->0) {
        cin >> data;
        head=myList.insertNodeAtHead(head, data);
    }

    myList.display(head);

    return 0;
}

