fn compress(chars: &mut Vec<char>) -> i32 {
    let length = chars.len();

    let compressed: &mut Vec<char> = &mut vec![];

    let mut count = 0;

    for i in 0..length {
        count += 1;

        if i + 1 == length || chars[i+1] != chars[i] {
            compressed.push(chars[i]);
            if count > 1 {
                compressed.extend(count.to_string().chars());
            }
            count = 0;
        }   
    }


    *chars = compressed.clone();

    compressed.len() as i32
}