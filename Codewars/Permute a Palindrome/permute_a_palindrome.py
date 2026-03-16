def permute_a_palindrome(input): 
    alphabet = {}

    for c in input:
        if alphabet.get(c):
            alphabet[c] += 1
        else:
            alphabet[c] = 1
            
    if len(alphabet) == 1:
        return True

    values = list(alphabet.values())
    if len([x for x in values if x % 2 != 0]) <= 1:
        return True
    else:
        return False
