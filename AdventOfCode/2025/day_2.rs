use std::fs;

fn parse_text() -> Vec<Vec<String>> {
    let contents = fs::read_to_string("\\path").expect("Should have been able to read the file");

    let mut content: Vec<Vec<String>> = vec![];

    let ct = contents
        .trim()
        .split(',')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for c in ct {
        let element = c.trim().split('-').map(|x| x.to_string()).collect();

        content.push(element);
    }

    content
}

fn is_redundant(ranges: Vec<Vec<String>>) -> i64 {
    let mut total: i64 = 0;
    for range in ranges {
        let n1: i64 = range[0].parse().unwrap_or(0);
        let n2: i64 = range[1].parse().unwrap_or(0);

        for i in n1..(n2 + 1) {
            let num = i.to_string();
            let length = num.to_string().len();

            let mut n = 1;
            while n < length {
                let subs = num
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(n)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<String>>();

                let first = subs.iter().next().unwrap();
                if subs.iter().all(|elem| elem == first) {
                    total += i;
                    break;
                } else {
                    n += 1;
                }
            }
        }
    }

    total
}

fn main() {
    let ranges = parse_text();
    let total = is_redundant(ranges);
    println!("{}", total);
}
