// Check if number of edits between two strings is not greater than 1. 

fn one_away(str1: String, str2: String) -> bool {
    str1.chars().filter(|x| !str2.contains(*x)).count() < 2
}
