fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut a_score = 0;
    let mut b_score = 0;

    a.iter().zip(b.iter())
        .map(|(a, b)| a - b)
        .for_each(|x| {
            if x > 0 {
                a_score += 1;
            } else if x < 0 {
                b_score += 1
            }
        });


    vec![a_score, b_score]
}
