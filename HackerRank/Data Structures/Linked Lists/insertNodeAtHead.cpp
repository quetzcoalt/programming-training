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
        head=myList.insert(head, data);
    }

    myList.display(head);

    return 0;
}

