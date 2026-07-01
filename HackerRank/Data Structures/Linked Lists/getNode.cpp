int getNode(Node *head, int positionFromTail) {
    Node *current = head;
    int length = 0;

    while (current->next) {
        length++;
        current = current->next;
    }

    cout << "Length: " << length << endl; 

    int position = length - positionFromTail;
    
    if (position == positionFromTail) return current->data;

    else {
        current = head;
        int count = 0;
        
        while (current->next) {
            if (position == count) return current->data;
            else {
                current = current->next;
                count++;
            }
        }

        return current->data;
    }
}
