use aoc::read_lines;
use std::io;

fn base(input: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let lines: Vec<String> = read_lines(input).unwrap();

    let lines: Vec<(i32, i32)> = lines
        .iter()
        .map(|l| l.split_whitespace().take(2).collect::<Vec<&str>>()) // "1  2" -> vec!["1", "2"]
        .map(|l| (l[0].parse::<i32>().unwrap(), l[1].parse::<i32>().unwrap())) // vec!["1", "2"] -> (1, 2)
        .collect();

    // vec![(1,2), (3,4)] -> vec![1,3] & vec![2,4]
    let (mut left, mut right): (Vec<_>, Vec<_>) = lines.into_iter().unzip();

    left.sort();
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
