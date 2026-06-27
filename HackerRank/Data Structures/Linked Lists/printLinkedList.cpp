/*
 * SinglyLinkedListNode {
 *     int data;
 *     SinglyLinkedListNode* next;
 * };
 *
 */
void printLinkedList(SinglyLinkedListNode* head) {
    SinglyLinkedListNode *start = head;
    
    while (start) {
        cout << start->data << endl;
        start = start->next;
    }
}
