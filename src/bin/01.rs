use aoc::read_lines;
use std::io;

fn base(input: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let lines: Vec<String> = read_lines(input).unwrap();

    let lines: Vec<Vec<&str>> = lines
        .iter()
        .map(|l| l.split_whitespace().take(2).collect::<Vec<&str>>())
        .collect();

    // converts split a string like "123   345", into vec!["123", "345"]
    let lines: Vec<(&str, &str)> = lines.into_iter().map(|l| (l[0], l[1])).collect();

    // converts vec![vec!["1", "2"], vec!["3", "4"]], into two separate vectors vec!["1", "3"] and
    // vec!["2", "4"]
    let (left, right): (Vec<_>, Vec<_>) = lines.into_iter().unzip();

    // parse string to int, and sort the vector
    let mut left = left
        .iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    left.sort();
    let mut right = right
        .iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    right.sort();

    Ok((left, right))
}

fn first(input: &str) -> io::Result<i32> {
    let (left, right) = base(input).unwrap();

    let mut distance: i32 = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        distance += (l - r).abs();
    }

    Ok(distance)
}

fn second(input: &str) -> io::Result<i32> {
    let (left, right) = base(input).unwrap();

    use std::collections::HashMap;

    let mut right_hash: HashMap<i32, i32> = HashMap::new();

    for i in right {
        *right_hash.entry(i).or_insert(0) += 1;
    }

    let mut distance: i32 = 0;

    for i in left {
        distance += i * right_hash.get(&i).unwrap_or(&0);
    }

    Ok(distance)
}

pub fn main() {
    assert_eq!(first("data/examples/01.txt").unwrap(), 11);
    assert_eq!(second("data/examples/01.txt").unwrap(), 31);
    println!("{}", first("data/inputs/01.txt").unwrap());
    println!("{}", second("data/inputs/01.txt").unwrap());
}
