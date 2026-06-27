/*
 * SinglyLinkedListNode {
 *     int data;
 *     SinglyLinkedListNode* next;
 * };
 *
 */
SinglyLinkedListNode* insertNodeAtTail(SinglyLinkedListNode* head, int data) {
    SinglyLinkedListNode *newNode = new SinglyLinkedListNode(data);
    newNode->next = NULL;
    
    SinglyLinkedListNode *current = head;
    
    if (head == NULL) return newNode;
    else {
        while (current->next) current = current->next;
        
        current->next = newNode;
    }
    
    return head;
}
