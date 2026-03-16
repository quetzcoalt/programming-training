fn minimumLoss(price: &[i64]) -> i32 {
    let length = price.len();

    let mut indexed_prices: Vec<(i64, usize)> =
        price.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    indexed_prices.sort_by_key(|&(p, _)| p);

    let mut n_difference = i64::MAX;

    for i in 0..(length - 1) {
        if indexed_prices.get(i + 1).unwrap().1 < indexed_prices.get(i).unwrap().1 {
            let diff = indexed_prices.get(i + 1).unwrap().0 - indexed_prices.get(i).unwrap().0;

            if diff < n_difference {
                n_difference = diff;
            }
        }
    }

    n_difference as i32
}
