bool compare_lists(Node* head1, Node* head2) {
    if (head1 == NULL && head2 == NULL) return true;

    while (head1 && head2) {
        if (head1->data != head2->data)
            return false;

        else {
            head1 = head1->next;
            head2 = head2->next;
        }
    }

    if ((head1 && !head2) || (!head1 && head2)) return false;

    return true;
}
