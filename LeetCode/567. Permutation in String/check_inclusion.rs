fn check_inclusion(s1: String, s2: String) -> bool {
    let s1_length = s1.len();

    let mut char_frequency = [0; 26];
    let char_offset = 'a' as usize;

    s1.chars()
        .for_each(|c| char_frequency[c as usize - char_offset] += 1);

    // checking for exact matches
    s2.chars()
        .take(s1_length)
        .for_each(|c| char_frequency[c as usize - char_offset] -= 1);

    if char_frequency.into_iter().all(|x| x == 0) {
        return true;
    }

    //checking for permutations
    let front = s2.chars();
    let back = s2.chars().skip(s1_length);

    for (o, i) in front.zip(back) {
        // incrementing characters out of sliding window, decreamenting characters in
        char_frequency[o as usize - char_offset] += 1;
        char_frequency[i as usize - char_offset] -= 1;

        if char_frequency.iter().all(|x| *x == 0) {
            return true;
        }
    }
    false
}
