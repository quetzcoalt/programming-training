Node* reverse(Node *head) {
    Node *current = head, *prev = NULL, *next;

    while (current != NULL) {
        next = current->next;

        current->next = prev;

        prev = current;
        current = next;
    }

    head = prev;

    return head;
}
